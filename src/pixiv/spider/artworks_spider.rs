use super::GlobalConfig;
use super::{
    super::base::{Artwork, PixivError},
    AsyncQueue, RunnerContext, StreamWrapper,
};
use futures::{stream::select_all, StreamExt};
use log::{error, info};
use mongodb::{bson::doc, bson::Document, Collection};
use std::sync::Arc;
struct UpdateArtworkTask {
    artwork_id: i64,
    is_new: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("")]
    EmptyQueue,
    #[error("抓取作品-网络错误")]
    OtherError(#[from] PixivError),
    #[error("{0:?} 作品不存在或被删除")]
    ArtworkNotExists(i64, bool),
}

async fn query_documents(
    collection: &mut Collection,
    pipe_line: Vec<mongodb::bson::Document>,
) -> Vec<Document> {
    let cursor = collection.aggregate(pipe_line, None).await.unwrap();
    let tasks = cursor
        .filter_map(|x| async move {
            match x {
                Err(_) => None,
                Ok(s) => Some(s),
            }
        })
        .collect()
        .await;
    tasks
}

async fn load_tasks(collection: &mut Collection, cache_size: usize) -> Vec<UpdateArtworkTask> {
    let mut cache = Vec::new();
    query_documents(
        collection,
        vec![
            mongodb::bson::doc! {"$match" : {"last_update_time" : {"$exists" : 0}}},
            mongodb::bson::doc! {"$limit" : cache_size as i64},
        ],
    )
    .await
    .into_iter()
    .for_each(|x| {
        let artwork_id = match x.get_i64("id") {
            Ok(x) => x,
            _ => return,
        };
        cache.push(UpdateArtworkTask {
            artwork_id,
            is_new: true,
        });
    });
    if cache.len() < cache_size {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 86400;
        query_documents(
            collection,
            vec![
                mongodb::bson::doc! {"$match" : {"last_update_time" : {"$exists" : 1}}},
                mongodb::bson::doc! {"$match" : {"last_update_time" : {"$lt" : time}}},
                mongodb::bson::doc! {"$limit" : cache_size as i64},
            ],
        )
        .await
        .into_iter()
        .for_each(|x| {
            let artwork_id = match x.get_i64("id") {
                Ok(x) => x,
                _ => return,
            };
            cache.push(UpdateArtworkTask {
                artwork_id,
                is_new: false,
            });
        });
    };
    cache
}

async fn fetch_artwork(
    mut ctx: RunnerContext<UpdateArtworkTask>,
) -> (
    Result<(Artwork, bool), Error>,
    RunnerContext<UpdateArtworkTask>,
) {
    let t = match ctx.queue.pop().await {
        Some(x) => x,
        None => return (Err(Error::EmptyQueue), ctx),
    };
    match ctx.client.load_artwork(t.artwork_id).await {
        Ok(x) => (Ok((x, t.is_new)), ctx),
        Err(PixivError::ArtworkNotExists(_)) => {
            (Err(Error::ArtworkNotExists(t.artwork_id, t.is_new)), ctx)
        }
        Err(e) => (Err(Error::OtherError(e)), ctx),
    }
}

pub async fn run(config: Arc<GlobalConfig>, mut collection: Collection) {
    let cache = Arc::new(AsyncQueue::new());
    let mut streams = Vec::new();
    let mut total_fill_count = 0;
    let mut total_update_count = 0;
    for _ in 0..config.update_artwork_thread_num {
        let context = RunnerContext {
            queue: cache.clone(),
            client: super::new_client(config.clone()).unwrap(),
        };
        streams.push(StreamWrapper::new(context, fetch_artwork));
    }
    let mut selector = select_all(streams);
    loop {
        if cache.size().await == 0 {
            cache
                .push_all(load_tasks(&mut collection, 10000).await)
                .await;
        }

        match selector.next().await.unwrap() {
            Err(Error::EmptyQueue) => info!("EmptyQueue"),
            Err(Error::OtherError(x)) => error!("{:?}", x),
            Err(Error::ArtworkNotExists(artwork_id, is_new)) => {
                if is_new {
                    collection
                        .delete_one(doc! {"id" : artwork_id}, None)
                        .await
                        .unwrap();
                    info!(
                        "{} 作品被删除或不存在",
                        artwork_id
                    );
                } else {
                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    collection
                        .update_one(
                            doc! {"id" : artwork_id},
                            doc! {"$set" : {"last_update_time" : time}},
                            None,
                        )
                        .await
                        .unwrap();
                    info!("{} 作品被删除或不存在，无法更新", artwork_id);
                }
            }
            Ok((mut artwork, is_new)) => {
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                artwork.last_update_time = Some(time as i64);
                let document = mongodb::bson::to_document(&artwork).unwrap();

                collection
                    .update_one(
                        doc! {"id" : artwork.artwork_id},
                        doc! {"$set" : document},
                        None,
                    )
                    .await
                    .unwrap();
                if is_new {
                    total_fill_count += 1;
                    if total_fill_count % 100 == 0 {
                        info!("新增了 {} 个作品", total_fill_count);
                    }
                } else {
                    total_update_count += 1;
                    if total_update_count % 100 == 0 {
                        info!("更新了 {} 个作品", total_update_count);
                    }
                }
            }
        };
    }
}
