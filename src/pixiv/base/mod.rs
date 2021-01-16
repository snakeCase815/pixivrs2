mod artwork;
mod artwork_db;
mod pixiv_client;
pub use artwork::{Artwork, PixivUser};
pub use pixiv_client::{PixivClient, PixivClientOption};

#[derive(thiserror::Error, Debug)]
pub enum PixivError {
    #[error("网络错误 : {0:?}")]
    ClientError(#[from] isahc::Error),
    #[error("IO错误 : {0:?}")]
    ClientIoError(#[from] std::io::Error),
    #[error("{0:?} 作品不存在或被删除")]
    ArtworkNotExists(i64),
    #[error("BadResponse {0}")]
    BadResponse(String, Vec<u8>),
    #[error("({0}) - JSON格式错误 : {1}")]
    ParseJSONError(String, String),
    #[error("({0}) - HTTP状态码错误 : {1}")]
    WrongHttpStatusCode(String, u16),
}
