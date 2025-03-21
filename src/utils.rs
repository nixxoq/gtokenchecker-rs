use std::collections::HashMap;

use chrono::DateTime;

pub enum StrOrInt {
    StrV(String),
    I32V(i32),
    I64V(i64),
    I128V(i128),
}
pub enum CdnType {
    UserAvatar,
    UserBanner,
    GuildIcon,
    GuildBanner,
}

pub enum ImageType {
    Png,
    Jpg,
    Jpeg,
    Webp,
    Gif,
    Svg,
}

const DISCORD_CDN_BASE: &str = "https://cdn.discordapp.com";

/*
TODO:

Migrate all exist functions on utils.rs into the Utils class
*/

pub struct Utils {}
impl Utils {
    pub fn gen_url(
        cdn_type: CdnType,
        type_id: &String,
        hash: &String,
        image_type: ImageType,
    ) -> String {
        let result = format!(
            "{}/{}/{}/{}.{}",
            DISCORD_CDN_BASE,
            cdn_type.as_str(),
            type_id,
            hash,
            image_type.as_str()
        );

        result
    }

    pub fn get_account_creation(snowflake_id: i64, format: Option<&str>) -> String {
        let user_creation = ((snowflake_id >> 22) + 1420070400000) / 1000;
        let user_creation = DateTime::from_timestamp(user_creation, 0)
            .unwrap()
            .format(format.unwrap_or("%d.%m.%Y %H:%M:%S"))
            .to_string();

        user_creation
    }

    pub fn get_string_value(
        dict: &HashMap<String, StrOrInt>,
        key: &str,
        default_value: Option<&str>,
    ) -> Option<String> {
        match dict.get(key) {
            Some(StrOrInt::StrV(value)) => Some(value.clone()),
            _ => Some(default_value.unwrap_or("").to_string()),
        }
    }
}

impl CdnType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CdnType::UserAvatar => "avatars",
            CdnType::UserBanner => "banners",
            CdnType::GuildIcon => "icons",
            CdnType::GuildBanner => "banners",
        }
    }
}

impl ImageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageType::Png => "png",
            ImageType::Jpg => "jpg",
            ImageType::Jpeg => "jpeg",
            ImageType::Webp => "webp",
            ImageType::Gif => "gif",
            ImageType::Svg => "svg",
        }
    }
}
