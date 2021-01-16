use super::{GlobalConfig, PixivClient};
use log::{error, info};
use mongodb::bson::doc;
use mongodb::Collection;
use std::sync::Arc;

#[derive(serde::Deserialize)]
struct TagConfig {
    keyword: String,
    types: Vec<String>,
    max_page: u32,
    sorts: Vec<String>,
}

#[derive(Debug)]
struct TagsSpiderResult {
    ids: Vec<i64>,
    tag: String,
    artwork_type: String,
    sort: String,
    last_page: usize,
}

async fn crawl_tags(
    tag: &str,
    artwork_type: &str,
    sort: &str,
    max_page: u32,
    client: &mut PixivClient,
) -> TagsSpiderResult {
    let mut ids = Vec::new();
    let mut page_num = 1;
    while page_num <= max_page {
        match client
            .search(tag, sort, artwork_type, page_num as u32)
            .await
        {
            Ok(x) => {
                let l = x.len();
                ids.extend(x);
                if l < 60 {
                    break;
                }
            }
            Err(e) => error!("{:?}", e),
        };
        page_num += 1;
    }
    TagsSpiderResult {
        ids: ids,
        tag: tag.to_string(),
        artwork_type: artwork_type.to_string(),
        sort: sort.to_string(),
        last_page: page_num as usize,
    }
}

pub async fn run(config: Arc<GlobalConfig>, collection: Collection) {
    let mut client = super::new_client(config.clone()).unwrap();
    loop {
        let tags_config = match serde_json::from_str::<Vec<TagConfig>>(
            &std::fs::read_to_string(&config.search_config_path).unwrap(),
        ) {
            Ok(x) => x,
            Err(e) => {
                error!("{:?}", e);
                async_std::task::sleep(std::time::Duration::from_secs(10)).await;
                continue;
            }
        };
        for tag_config in tags_config {
            for _type in &tag_config.types {
                for _sort in &tag_config.sorts {
                    let r = crawl_tags(
                        &tag_config.keyword,
                        _type,
                        _sort,
                        tag_config.max_page,
                        &mut client,
                    )
                    .await;
                    let mut inserted_count: usize = 0;
                    for _id in &r.ids {
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
                        "{}-{}-{} 共 {} 页 , {} 个作品 , 新增了 {}",
                        tag_config.keyword,
                        _type,
                        _sort,
                        r.last_page,
                        r.ids.len(),
                        inserted_count
                    );
                }
            }
        }
    }
}
