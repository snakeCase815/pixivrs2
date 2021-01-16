use super::Artwork;
use super::PixivError;
use log::debug;
use std::convert::TryFrom;
use isahc::prelude::*;
use futures::AsyncReadExt;

type Result<T> = std::result::Result<T, PixivError>;

macro_rules! JSON_GET {
    ($x : expr,$v: expr,$cookie : expr) => {
        match $x.get($v) {
            Some(x) => x,
            None => {
                return Err(PixivError::ParseJSONError(
                    $cookie,
                    format!("字段不存在: {},'{:?}'", $v, serde_json::to_string($x)),
                )
                .into())
            }
        }
    };
}
pub struct PixivClientOption {
    _proxy: Option<String>,
    _language: String,
    _cookie: String,
    _ua: String,
    _country: String,
}

impl PixivClientOption {
    pub fn new() -> PixivClientOption {
        PixivClientOption {
            _proxy: None,
            _language: "zh".into(),
            _cookie: "".into(),
            _ua: "".into(),
            _country: "CN".into(),
        }
    }
    pub fn proxy(mut self, proxy: &str) -> PixivClientOption {
        self._proxy = Some(proxy.to_string());
        self
    }
    pub fn language(mut self, lang: &str) -> PixivClientOption {
        self._language = lang.into();
        self
    }
    pub fn cookie(mut self, cookie: &str) -> PixivClientOption {
        self._cookie = cookie.into();
        self
    }
    pub fn useragent(mut self, ua: &str) -> PixivClientOption {
        self._ua = ua.into();
        self
    }
}

pub struct PixivClient {
    _client: isahc::HttpClient,
    _options: PixivClientOption,
}

fn parse_detail_page(content: &str) -> Option<String> {
    let start_flag = "<meta name=\"preload-data\" id=\"meta-preload-data\" content='";
    let end_flag = "'>";
    let start_pos = content.find(start_flag)? + start_flag.len();
    let end_pos = content[start_pos..].find(end_flag)? + start_pos;
    Some(content[start_pos..end_pos].to_string())
}

fn decompress_gzip(data: &[u8]) -> Option<Vec<u8>> {
    // use std::io::Read;
    // let mut decompressor = flate2::read::GzDecoder::new(data);
    // let mut buffer: Vec<u8> = Vec::new();
    // decompressor
    //     .read_to_end(&mut buffer)
    //     .map_or(None, |x| Some(x))?;
    Some(data.to_vec())
}

impl PixivClient {
    pub fn new_with_option(option: PixivClientOption) -> Result<PixivClient> {
        let mut c = isahc::HttpClientBuilder::new();
        c = c.timeout(std::time::Duration::from_secs(10));
        c = c.ssl_options(isahc::config::SslOption::DANGER_ACCEPT_INVALID_CERTS | isahc::config::SslOption::DANGER_ACCEPT_INVALID_HOSTS);
        

        let mut default_headers = http::HeaderMap::new();
        c = c.default_header("sec-fetch-dest","empty");
        c = c.default_header("sec-fetch-mode","cors");
        c = c.default_header("sec-fetch-site","none");
        c = c.default_header("accept-encoding","gzip, deflate, br");
        c = c.default_header("accept-language",format!(
            "{}-{},jp;q=0.9",
            &option._language, &option._country
        ));
        c = c.default_header("accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9");
        c = c.default_header("user-agent",option._ua.clone());
        match option._proxy {
            Some(ref x) => c = c.proxy(Some(x.parse().unwrap())),
            _ => (),
        };
        Ok(PixivClient {
            _client: c.build()?,
            _options: option,
        })
    }

    pub async fn load_artwork(&mut self, pixiv_id: i64) -> Result<Artwork> {
        let error_cookie = format!("load_artwork-{}", pixiv_id);
        let url = format!(
            "https://www.pixiv.net/artworks/{}?lang={}",
            pixiv_id, self._options._language
        );
        let mut response = self._client.get_async(&url).await?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let mut bytes_content = Vec::new();
                response.body_mut().read_to_end(&mut bytes_content).await?;
                let content = match decompress_gzip(&bytes_content) {
                    Some(x) => x,
                    None => return Err(PixivError::BadResponse(url, bytes_content)),
                };
                let content = if let Ok(x) = String::from_utf8(content) {
                    x
                } else {
                    return Err(PixivError::BadResponse(url, bytes_content));
                };

                let parse_result = parse_detail_page(&content).map_or(
                    Err(PixivError::BadResponse(url, content.as_bytes().to_vec())),
                    |x| Ok(x),
                )?;

                let json_value: serde_json::Value = match serde_json::from_str(&parse_result) {
                    Ok(x) => x,
                    Err(_) => return Err(PixivError::ParseJSONError(error_cookie, parse_result)),
                };
                let illust_value = JSON_GET!(&json_value, "illust", error_cookie);
                let illust_value = JSON_GET!(illust_value, &format!("{}", pixiv_id), error_cookie);
                let artwork = match Artwork::try_from(illust_value) {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(PixivError::ParseJSONError(error_cookie, e.0.to_string()))
                    }
                };
                return Ok(artwork);
            }
            404 => return Err(PixivError::ArtworkNotExists(pixiv_id)),
            _ => return Err(PixivError::WrongHttpStatusCode(error_cookie, status_code)),
        }
    }

    pub async fn load_by_creator(&mut self, creator_id: i64) -> Result<Vec<i64>> {
        let mut result = Vec::new();
        let url = format!("https://www.pixiv.net/ajax/user/{}/profile/all", creator_id);
        let error_cookie = format!("load_by_creator_{}", creator_id);
        let mut response = self._client.get_async(&url).await?;
        let status_code = response.status().as_u16();
        if status_code != 200 {
            return Err(PixivError::WrongHttpStatusCode(error_cookie, status_code));
        }
        let mut bytes_content = Vec::new();
        response.body_mut().read_to_end(&mut bytes_content).await?;
        let content = match decompress_gzip(&bytes_content) {
            Some(x) => x,
            None => return Err(PixivError::BadResponse(url, bytes_content)),
        };
        let content = if let Ok(x) = String::from_utf8(content) {
            x
        } else {
            return Err(PixivError::BadResponse(url, bytes_content));
        };
        let json_value: serde_json::Value = match serde_json::from_str(&content) {
            Ok(x) => x,
            Err(_) => return Err(PixivError::ParseJSONError(error_cookie, content)),
        };
        let body = JSON_GET!(&json_value, "body", error_cookie);
        let empty_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let illusts = JSON_GET!(body, "illusts", error_cookie)
            .as_object()
            .unwrap_or(&empty_map);
        let manga = JSON_GET!(body, "manga", error_cookie)
            .as_object()
            .unwrap_or(&empty_map);
        for key in illusts.keys() {
            let artwork_id = match i64::from_str_radix(key, 10) {
                Ok(x) => x,
                Err(_) => {
                    debug!("[{}] 类型错误: {}", error_cookie, key);
                    continue;
                }
            };
            result.push(artwork_id);
        }
        for key in manga.keys() {
            let artwork_id = match i64::from_str_radix(key, 10) {
                Ok(x) => x,
                Err(_) => {
                    debug!("[{}] 类型错误: {}", error_cookie, key);
                    continue;
                }
            };
            result.push(artwork_id);
        }
        Ok(result)
    }
    pub async fn download_image(&mut self,url : &str) -> Result<Vec<u8>> {
        let referer = "https://www.pixiv.net/".to_string();
        let req = isahc::http::Request::builder()
            .header("Referer",referer)
            .uri(url)
            .method(isahc::http::Method::GET)
            .body(()).unwrap();
        let mut resp = self._client.send_async(req).await?;
        let status_code = resp.status().as_u16();
        if status_code != 200 {
            return Err(PixivError::WrongHttpStatusCode("(download_img)".to_string(), status_code));
        }
        let mut bytes_content = Vec::new();
        resp.body_mut().read_to_end(&mut bytes_content).await?;
        Ok(bytes_content)
    }
    pub async fn search(
        &mut self,
        tag: &str,
        sort: &str,
        _type: &str,
        page: u32,
    ) -> Result<Vec<i64>> {
        let error_cookie = format!("({}-{}-{}-{})", tag, sort, _type, page);
        let (url, json_key) = if _type == "manga" {
            (format!("https://www.pixiv.net/ajax/search/manga/{}?word={0}&order={}&mode=all&p={}&s_mode=s_tag_full&type=illust_and_ugoira&lang=zh"
                    ,urlencoding::encode(tag),sort,page),"manga")
        } else {
            (format!("https://www.pixiv.net/ajax/search/illustrations/{}?word={0}&order={}&mode=all&p={}&s_mode=s_tag_full&type=illust_and_ugoira&lang=zh"
                    ,urlencoding::encode(tag),sort,page),"illust")
        };
        // let url = format!("{}",urlencoding::encode(tag));
        let mut response = self._client.get_async(&url).await?;
        let status_code = response.status().as_u16();
        if status_code != 200 {
            return Err(PixivError::WrongHttpStatusCode(error_cookie, status_code));
        }
        let mut bytes_content = Vec::new();
        response.body_mut().read_to_end(&mut bytes_content).await?;
        let content = match decompress_gzip(&bytes_content) {
            Some(x) => x,
            None => return Err(PixivError::BadResponse(url, bytes_content)),
        };
        let content = if let Ok(x) = String::from_utf8(content) {
            x
        } else {
            return Err(PixivError::BadResponse(url, bytes_content));
        };
        let json_value: serde_json::Value = match serde_json::from_str(&content) {
            Ok(x) => x,
            Err(_) => return Err(PixivError::ParseJSONError(error_cookie, content)),
        };
        let illusts_json = JSON_GET!(&json_value, "body", error_cookie);
        let illusts_json = JSON_GET!(&illusts_json, json_key, error_cookie);
        let illusts_json = JSON_GET!(&illusts_json, "data", error_cookie);
        let illusts_json = match illusts_json.as_array() {
            Some(x) => x,
            None => {
                return Err(PixivError::ParseJSONError(
                    error_cookie,
                    "body类型错误".to_string(),
                ))
            }
        };
        let mut artworks = Vec::new();
        for illust in illusts_json {
            let artwork_id = if let Some(x) = illust.get("id") {
                if let Some(v) = x.as_str() {
                    if let Ok(vi) = i64::from_str_radix(v, 10) {
                        vi
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            artworks.push(artwork_id);
        }
        Ok(artworks)
    }
}
