pub mod artworks_spider;
pub mod authors_spider;
pub mod stream_wrapper;
pub mod tags_spider;
pub use super::base::{Artwork, PixivClient, PixivClientOption, PixivError, PixivUser};
pub use super::config::GlobalConfig;
pub use stream_wrapper::{AsyncQueue, RunnerContext, StreamWrapper};

pub fn new_client(
    config: std::sync::Arc<GlobalConfig>,
) -> Result<super::base::PixivClient, super::base::PixivError> {
    super::base::PixivClient::new_with_option(
        super::base::PixivClientOption::new()
            .cookie(&config.pixiv_cookie)
            .proxy(&config.proxy)
            .useragent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.75 Safari/537.36 Edg/86.0.622.38")
            .language("zh"),
    )
}
