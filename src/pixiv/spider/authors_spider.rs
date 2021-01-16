use super::super::base::{PixivError, PixivUser};
use super::super::config::GlobalConfig;
use super::stream_wrapper::{AsyncQueue, RunnerContext, StreamWrapper};
use futures::StreamExt;
use log::{error, info};
use mongodb::{bson::doc, Collection};
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("")]
    EmptyQueue,
    #[error("抓取作者-网络错误")]
    OtherError(PixivError, PixivUser),
}

pub async fn load_authors(collection: &mut Collection) -> Vec<PixivUser> {
    let pipe_line = vec![
        doc! {"$match" : {"user" : {"$exists" : 1}}},
        doc! {"$group":{"_id":"$user.id","id":{"$first":"$user.id"},"name":{"$first":"$user.name"}}},
    ];
    let cursor = collection.aggregate(pipe_line, None).await.unwrap();
    cursor
        .filter_map(|x| async {
            match x {
                Err(_) => None,
                Ok(u) => Some(mongodb::bson::from_document::<PixivUser>(u).unwrap()),
            }
        })
        .collect::<Vec<PixivUser>>()
        .await
}

pub async fn run(config: Arc<GlobalConfig>, mut collection: Collection) {
    let queue: Arc<AsyncQueue<PixivUser>> = Arc::new(AsyncQueue::new());
    let future_fn = async move |mut ctx: RunnerContext<PixivUser>| {
        let x = match ctx.queue.pop().await {
            Some(x) => x,
            None => return (Err(Error::EmptyQueue), ctx),
        };
        match ctx.client.load_by_creator(x.user_id.unwrap()).await {
            Ok(v) => (Ok((v, x)), ctx),
            Err(e) => (Err(Error::OtherError(e, x)), ctx),
        }
    };
    let mut streams = Vec::new();
    for _ in 0..config.user_detail_thread_num {
        streams.push(StreamWrapper::new(
            RunnerContext {
                queue: queue.clone(),
                client: super::new_client(config.clone()).unwrap(),
            },
            future_fn,
        ));
    }
    let mut selector = futures::stream::select_all(streams);
    loop {
        if queue.size().await <= 0 {
            queue.push_all(load_authors(&mut collection).await).await;
        }
        match selector.next().await.unwrap() {
            Err(e) => error!("{:?}", e),
            Ok((v, u)) => {
                let mut inserted_count: usize = 0;
                for _id in &v {
                    let mut options = mongodb::options::UpdateOptions::default();
                    options.upsert = Some(true);
                    let update_result = collection
                        .update_one(doc! {"id":_id}, doc! {"$set":{"id":_id}}, options)
                        .await
                        .unwrap();
                    if update_result.upserted_id.is_some() {
                        inserted_count += 1;
                    }
                }
                info!(
                    "作者 {:?}-{:?} 共 {} 个作品,新增了 {} 个作品",
                    u.user_id,
                    u.name,
                    v.len(),
                    inserted_count
                );
            }
        };
    }
}
