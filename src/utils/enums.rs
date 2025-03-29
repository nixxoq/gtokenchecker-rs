use reqwest::StatusCode;
use std::{error::Error, fmt};

use super::structs::UnauthorizedResponse;

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

#[derive(Debug)]
pub enum ApiError {
    Unauthorized(UnauthorizedResponse),
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    UnexpectedStatus(StatusCode, String),
    ClientBuildError(reqwest::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Unauthorized(resp) => {
                write!(f, "Unauthorized ({}): {}", resp.code, resp.message)
            }
            ApiError::RequestError(err) => write!(f, "Request error: {}", err),
            ApiError::ParseError(err) => write!(f, "JSON parse error: {}", err),
            ApiError::UnexpectedStatus(status, body) => {
                write!(f, "Unexpected status code: {}. Body: {}", status, body)
            }
            ApiError::ClientBuildError(err) => write!(f, "Failed to build HTTP client: {}", err),
        }
    }
}

// extend std::error:Error trait
impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiError::RequestError(err) => Some(err),
            ApiError::ParseError(err) => Some(err),
            ApiError::ClientBuildError(err) => Some(err),
            ApiError::Unauthorized(_) => None,
            ApiError::UnexpectedStatus(_, _) => None,
        }
    }
}

// reqwest::Error trait
impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::RequestError(err)
    }
}

// serde_json::Error trait
impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::ParseError(err)
    }
}
