use serde::Deserialize;

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub mongo_url: String,
    pub database: String,
    pub collection: String,
    pub proxy: String,
    pub pixiv_cookie: String,
    pub search_config_path: String,
    pub search_thread_num: u32,
    pub user_detail_thread_num: u32,
    pub update_artwork_thread_num: u32,
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_CONFIG : std::sync::Arc<GlobalConfig> = {
        std::sync::Arc::new(serde_json::from_str::<GlobalConfig>(
            &std::fs::read_to_string("config.json").unwrap(),
        )
        .unwrap())
    };
}
