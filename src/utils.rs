use std::collections::HashMap;

use chrono::DateTime;
use constants::DISCORD_CDN_BASE;
use enums::{BannerType, CdnType, ImageType, StrOrInt};

pub mod constants;
pub mod enums;
pub mod macros;
pub mod structs;

fn gen_url(cdn_type: CdnType, type_id: &String, hash: &String, image_type: ImageType) -> String {
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
pub struct Utils;
impl Utils {
    pub fn gen_url(
        cdn_type: CdnType,
        type_id: &String,
        hash: &String,
        image_type: ImageType,
    ) -> String {
        gen_url(cdn_type, type_id, hash, image_type)
    }

    /// Generates the URL for a user avatar from Discord's CDN
    ///
    /// This is a wrapper around the `gen_url` function for User Avatars.
    ///
    /// # Arguments
    ///
    /// * `type_id`: A string representing the type ID of the user.
    /// * `hash`: A string representing the hash of the user's avatar.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL to the user's avatar on Discord's CDN.
    ///
    pub fn get_avatar(type_id: &String, hash: &String) -> String {
        Self::gen_url(
            CdnType::UserAvatar,
            type_id,
            hash,
            if hash.to_lowercase().starts_with("a_") {
                ImageType::Gif
            } else {
                ImageType::Png
            },
        )
    }

    /// Generates the URL for a user/guild banner from Discord's CDN
    ///
    /// This is a wrapper around the `gen_url` function for User/Guild Banners.
    ///
    /// # Arguments
    ///
    /// * `type_id`: A string representing the type ID of the user.
    /// * `hash`: A string representing the hash of the user's avatar.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL to the user/guild banner on Discord's CDN.
    ///
    pub fn get_banner(banner_type: BannerType, type_id: &String, hash: &String) -> String {
        Self::gen_url(
            CdnType::Banner(banner_type),
            type_id,
            hash,
            hash.starts_with("a_")
                .then_some(ImageType::Gif)
                .or(ImageType::Png.into())
                .unwrap(),
        )
    }

    /// Extracts account creation date from Snowflake and then convert to humanly format
    ///
    ///
    /// # Arguments
    ///
    /// * `snowflake_id`: type id of User (snowflake)
    /// * `format`: Strftime string to format (defaults `"%d.%m.%Y %H:%M:%S"`)
    ///
    /// # Returns
    ///
    /// A `String` User account creation
    ///
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
            CdnType::Banner(BannerType::Guild | BannerType::User) => "banners",
            CdnType::GuildIcon => "icons",
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
