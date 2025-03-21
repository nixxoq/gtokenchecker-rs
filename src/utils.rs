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
    GuildIcon,
    Banner(BannerType),
}

pub enum BannerType {
    User,
    Guild,
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
pub const USER_FLAGS: &[(i128, &str)] = &[
    (1 << 0, "Staff"),
    (1 << 1, "Guild Partner"),
    (1 << 2, "HypeSquad Events Member"),
    (1 << 3, "Bug Huner Level 1"),
    (1 << 4, "SMS 2FA Enabled"),
    (1 << 5, "Dismissed Nitro promotion"),
    (1 << 6, "House Bravery Member"),
    (1 << 7, "House Brilliance Member"),
    (1 << 8, "House Balance Member"),
    (1 << 9, "Early Nitro Supporter"),
    (1 << 10, "Team Supporter"),
    (1 << 13, "Unread urgent system messages"),
    (1 << 14, "Bug Hunter Level 2"),
    (1 << 15, "Under age account"),
    (1 << 16, "Verified Bot"),
    (1 << 17, "Early Verified Bot Developer"),
    (1 << 18, "Moderator Programs Alumni"),
    (1 << 19, "Bot uses only http interactions"),
    (1 << 20, "Marked as spammer"),
    (1 << 22, "Active Developer"),
    (1 << 23, "Provisional Account"),
    (1 << 33, "Global ratelimit"), // User has their global ratelimit raised to 1,200 requests per second
    (1 << 34, "Deleted account"),
    (1 << 35, "Disabled for suspicious activity"),
    (1 << 36, "Self-deleted account"),
    (1 << 41, "User account is disabled"),
];

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

pub struct Utils {}
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
        gen_url(
            CdnType::UserAvatar,
            type_id,
            hash,
            hash.to_lowercase()
                .starts_with("a_")
                .then_some(ImageType::Gif)
                .or(Some(ImageType::Png))
                .unwrap(),
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
        format!(
            "{}/{}/{}/{}.{}",
            DISCORD_CDN_BASE,
            CdnType::Banner(banner_type).as_str(),
            type_id,
            hash,
            hash.starts_with("a_")
                .then_some(ImageType::Gif.as_str())
                .or(Some(ImageType::Png.as_str()))
                .unwrap()
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
