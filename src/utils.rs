use std::{collections::HashMap, fs, path::Path, time::Duration};

use chrono::{DateTime, Utc};
use constants::{DISCORD_CDN_BASE, USER_FLAGS, USER_PERMISSIONS};
use enums::{BannerType, CdnType, ImageType, StrOrInt};

pub mod constants;
pub mod enums;
pub mod macros;
pub mod structs;

pub struct Utils;

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

    /// Reads Discord tokens from the provided input string.
    ///
    /// This function determines if the input string represents a valid file path.
    /// - If it's a file path, it reads the file content, extracting one token per non-empty line
    ///   (leading/trailing whitespace is trimmed).
    /// - If it's not a file path, the input string itself is treated as a single token
    ///
    /// # Arguments
    ///
    /// * `input`: A string slice (`&str`) which is either a path to a text file
    ///   containing tokens (one per line) or a single token string.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)`: A vector containing the extracted token strings if successful.
    ///   If the input was not a file, this vector will contain exactly one element.
    /// * `Err(String)`: An error message string if:
    ///     - The input is a file path, but the file cannot be read (e.g., permissions, not found).
    ///     - The input is a file path, but the file is empty or contains only whitespace lines.
    ///     - The input is not a file path, and the trimmed input string is empty.
    ///
    pub fn read_tokens_from_input(input: &str) -> Result<Vec<String>, String> {
        // TODO: include REGEX pattern to search discord tokens in txt file
        let path = Path::new(input);
        if path.is_file() {
            fs::read_to_string(path)
                .map_err(|e| format!("Failed reading file '{}': {}", input, e))
                .and_then(|content| {
                    let tokens: Vec<String> = content
                        .lines()
                        .map(str::trim)
                        .filter(|l| !l.is_empty())
                        .map(String::from)
                        .collect();

                    if tokens.is_empty() {
                        Err(format!("Token file is empty: {}", path.display()))
                    } else {
                        Ok(tokens)
                    }
                })
        } else {
            let token = input.trim();
            if token.is_empty() {
                Err("Provided token string is empty.".to_string())
            } else {
                Ok(vec![token.to_string()])
            }
        }
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

    /// Formats an ISO-like time string into a human-readable format
    ///
    ///
    /// # Arguments
    ///
    /// * `time_str`: date (String) to format (ex. "2024-08-30T23:41:13.947000+00:00")
    /// * `format`: strftime string to format (defaults `"%d.%m.%Y %H:%M:%S"`)
    ///
    /// # Returns
    ///
    /// A string containing the formatted time if `time_str` argument was successfully converted into the `chrono::DateTime` class.
    /// Otherwise, returns the original `time_str` if parsing fails
    pub fn format_time(time_str: &String, format: Option<&str>) -> String {
        let parsed_time = time_str.parse::<DateTime<Utc>>().ok();
        if let Some(time) = parsed_time {
            time.format(format.unwrap_or("%d.%m.%Y %H:%M:%S"))
                .to_string()
        } else {
            time_str.to_string()
        }
    }

    /// Retrieves a string value associated with a key from a HashMap<String, StrOrInt>.
    ///
    /// Note: This function always returns `Some(String)`, never `None`.
    ///
    /// # Arguments
    ///
    /// * `dict`: A reference to the HashMap containing `String` keys and `StrOrInt` values.
    /// * `key`: The string slice representing the key to look up.
    /// * `default_value`: An optional string slice to return if the key is not found
    ///   or the value is not a `StrV`. Defaults to `""` if `None`.
    ///
    /// # Returns
    ///
    /// * `Some(String)`: Containing either the found string value or the default value.
    ///
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

    /// Masks the last part of a string (aka discord token) separated by dots (`.`).
    ///
    /// # Arguments
    ///
    /// * `token`: The input string slice (`&str`) to mask.
    ///
    /// # Returns
    ///
    /// * `String`: The potentially masked string.
    ///
    pub fn mask_last_part(token: &str) -> String {
        let mut parts: Vec<_> = token.split(".").map(|part| part.to_owned()).collect();
        match parts.last_mut() {
            Some(last) => {
                *last = "*".repeat(last.len());
                parts.join(".")
            }
            _ => token.to_owned(),
        }
    }

    /// Returns a vector of strings containing the user flags given the public flags.
    ///
    /// # Arguments
    ///
    /// * `public_flags`: The public flags as an `i128` from the User object.
    ///
    /// # Returns
    ///
    /// * `Vec<String>`: A vector of strings containing the user flags as strings.
    pub fn get_user_flags(public_flags: i128) -> Vec<String> {
        USER_FLAGS
            .iter()
            .filter_map(|&(key, value)| {
                if (public_flags & key) == key && key != 0 {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Parses a string slice as a duration in seconds.
    ///
    /// This function attempts to parse the input string as an integer, which is then
    /// converted to a `Duration` object representing that many seconds.
    ///
    /// # Arguments
    ///
    /// * `arg`: A string slice (`&str`) representing the number of seconds.
    ///
    /// # Returns
    ///
    /// * `Ok(Duration)`: A `Duration` object if the string is successfully parsed.
    /// * `Err(std::num::ParseIntError)`: An error if the string cannot be parsed as an integer.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input string is not a valid representation of an integer.
    pub fn parse_duration_secs(arg: &str) -> Result<Duration, std::num::ParseIntError> {
        let seconds = arg.parse()?;
        Ok(Duration::from_secs(seconds))
    }

    /// Returns a vector of strings containing the user permissions given the user's permissions.
    ///
    /// This function takes a string slice representing the user's permissions as an `i128`
    /// and returns a vector of strings containing the user permissions as strings.
    ///
    /// # Arguments
    ///
    /// * `user_permissions`: A string slice (`&str`) representing the user's permissions as an `i128`.
    ///
    /// # Returns
    ///
    /// * `Vec<String>`: A vector of strings containing the user permissions as strings.
    pub fn get_user_permissions(user_permissions: &str) -> Vec<String> {
        let user_permissions = user_permissions.parse::<i128>().unwrap_or(0);
        USER_PERMISSIONS
            .iter()
            .filter_map(|&(key, value)| {
                if (user_permissions & key) == key {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl CdnType {
    /// Returns a string representation of the `CdnType` enum as a path for a Discord CDN URL.
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            CdnType::UserAvatar => "avatars",
            CdnType::Banner(BannerType::Guild | BannerType::User) => "banners",
            CdnType::GuildIcon => "icons",
        }
    }
}

impl ImageType {
    /// Returns a string representation of the `ImageType` enum as a file extension for a Discord CDN URL.
    pub fn as_str<'a>(&self) -> &'a str {
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
