use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct FromError(pub String);
#[derive(Serialize, Deserialize, Debug)]
pub struct Artwork {
    #[serde(
        rename(serialize = "_id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub _id: Option<ObjectId>,
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub artwork_id: i64,
    #[serde(
        rename(serialize = "caption", deserialize = "caption"),
        skip_serializing_if = "Option::is_none"
    )]
    pub caption: Option<String>,
    #[serde(
        rename(serialize = "create_date", deserialize = "create_date"),
        skip_serializing_if = "Option::is_none"
    )]
    pub create_date: Option<String>,
    #[serde(
        rename(serialize = "type", deserialize = "type"),
        skip_serializing_if = "Option::is_none"
    )]
    pub artwork_type: Option<String>,
    #[serde(
        rename(serialize = "height", deserialize = "height"),
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i32>,
    #[serde(
        rename(serialize = "width", deserialize = "width"),
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i32>,
    #[serde(
        rename(serialize = "sanity_level", deserialize = "sanity_level"),
        skip_serializing_if = "Option::is_none"
    )]
    pub sanity_level: Option<i32>,
    #[serde(
        rename(serialize = "title", deserialize = "title"),
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        rename(serialize = "total_bookmarks", deserialize = "total_bookmarks"),
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bookmarks: Option<i32>,
    #[serde(
        rename(serialize = "total_view", deserialize = "total_view"),
        skip_serializing_if = "Option::is_none"
    )]
    pub total_view: Option<i32>,
    #[serde(
        rename(serialize = "last_update_time", deserialize = "last_update_time"),
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_time: Option<i64>,
    #[serde(
        rename(serialize = "image_urls", deserialize = "image_urls"),
        skip_serializing_if = "Option::is_none"
    )]
    pub image_urls: Option<PixivImageUrls>,
    #[serde(
        rename(serialize = "user", deserialize = "user"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<PixivUser>,
    #[serde(
        rename(serialize = "tags", deserialize = "tags"),
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Vec<PixivTag>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PixivImageUrls {
    #[serde(
        rename(serialize = "medium", deserialize = "medium"),
        skip_serializing_if = "Option::is_none"
    )]
    pub medium: Option<String>,
    #[serde(
        rename(serialize = "square_medium", deserialize = "square_medium"),
        skip_serializing_if = "Option::is_none"
    )]
    pub square_medium: Option<String>,
    #[serde(
        rename(serialize = "large", deserialize = "large"),
        skip_serializing_if = "Option::is_none"
    )]
    pub large: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PixivUser {
    #[serde(
        rename(serialize = "account", deserialize = "account"),
        skip_serializing_if = "Option::is_none"
    )]
    pub account: Option<String>,
    #[serde(
        rename(serialize = "id", deserialize = "id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<i64>,
    #[serde(
        rename(serialize = "name", deserialize = "name"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PixivTag {
    #[serde(rename(serialize = "name", deserialize = "name"))]
    pub name: String,
    #[serde(
        rename(serialize = "translated_name", deserialize = "translated_name"),
        skip_serializing_if = "Option::is_none"
    )]
    pub trans: Option<String>,
}

use std::convert::TryFrom;

macro_rules! JSON_GET {
    ($json : expr,$field : expr,$type : ident) => {
        match $json.get($field) {
            Some(x) => match x.$type() {
                Some(x) => Some(x),
                None => return Err(FromError(format!("类型转换错误: {}", $field))),
            },
            None => None,
        }
    };

    ($json : expr,$field : expr,$type : ident,$map_f : expr) => {
        match $json.get($field) {
            Some(x) => match x.$type() {
                Some(x) => Some(x).map($map_f),
                None => return Err(FromError(format!("类型转换错误: {}", $field))),
            },
            None => None,
        }
    };
}
impl TryFrom<&serde_json::Value> for Artwork {
    type Error = FromError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let artwork_id = match JSON_GET!(value, "id", as_str) {
            Some(x) => match i64::from_str_radix(x, 10) {
                Ok(n) => n,
                Err(_) => return Err(FromError(format!("ID类型错误：{}", x))),
            },
            None => return Err(FromError("artwork_id must be exists".to_string())),
        };
        let caption = JSON_GET!(value, "illustComment", as_str, |x| x.to_string());
        let create_date = JSON_GET!(value, "createDate", as_str, |x| x.to_string());
        let height = JSON_GET!(value, "height", as_i64, |x| x as i32);
        let sanity_level = JSON_GET!(value, "sl", as_i64, |x| x as i32);
        let title = JSON_GET!(value, "title", as_str, |x| x.to_string());
        let total_bookmarks = JSON_GET!(value, "bookmarkCount", as_i64, |x| x as i32);
        let total_view = JSON_GET!(value, "viewCount", as_i64, |x| x as i32);
        let width = JSON_GET!(value, "width", as_i64, |x| x as i32);
        let artwork_type = JSON_GET!(value, "illustType", as_i64, |x| {
            match x {
                0 => "illust".to_string(),
                1 => "manga".to_string(),
                _ => "unknow".to_string(),
            }
        });
        let user = match PixivUser::try_from(value) {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        let images_urls = if let Some(x) = value.get("urls") {
            match PixivImageUrls::try_from(x) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        } else {
            None
        };
        let tags = if let Some(tags_f) = value.get("tags") {
            if tags_f.get("tags").is_none() {
                None
            } else {
                match tags_f.get("tags").unwrap().as_array() {
                    Some(array) => {
                        let mut tags_array = Vec::new();
                        for item in array {
                            let tag = PixivTag::try_from(item);
                            if tag.is_ok() {
                                tags_array.push(tag.unwrap());
                            }
                        }
                        Some(tags_array)
                    }
                    None => None,
                }
            }
        } else {
            None
        };

        Ok(Artwork {
            _id: None,
            artwork_id: artwork_id,
            caption: caption,
            create_date: create_date,
            height: height,
            sanity_level: sanity_level,
            title: title,
            total_bookmarks: total_bookmarks,
            total_view: total_view,
            width: width,
            image_urls: images_urls,
            user: user,
            tags: tags,
            last_update_time: None,
            artwork_type: artwork_type,
        })
    }
}

impl TryFrom<&serde_json::Value> for PixivImageUrls {
    type Error = FromError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(PixivImageUrls {
            medium: JSON_GET!(value, "regular", as_str, |x| x.to_string()),
            square_medium: JSON_GET!(value, "thumb", as_str, |x| x.to_string()),
            large: JSON_GET!(value, "original", as_str, |x| x.to_string()),
        })
    }
}

impl TryFrom<&serde_json::Value> for PixivUser {
    type Error = FromError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let user_id = match JSON_GET!(value, "userId", as_str) {
            Some(x) => match i64::from_str_radix(x, 10) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        };
        Ok(PixivUser {
            account: JSON_GET!(value, "userAccount", as_str, |x| x.to_string()),
            user_id: user_id,
            name: JSON_GET!(value, "userName", as_str, |x| x.to_string()),
        })
    }
}

impl TryFrom<&serde_json::Value> for PixivTag {
    type Error = FromError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let tag_name = match JSON_GET!(value, "tag", as_str, |x| x.to_string()) {
            Some(x) => x,
            None => return Err(FromError("tagName必须存在".to_string())),
        };
        let trans = if let Some(x) = value.get("translation") {
            match x.as_object() {
                Some(map) => {
                    if let Some(v) = map.get("en") {
                        v.as_str().map(|a| a.to_string())
                    } else {
                        None
                    }
                }
                None => None,
            }
        } else {
            None
        };
        Ok(PixivTag {
            name: tag_name,
            trans: trans,
        })
    }
}
