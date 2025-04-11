use reqwest::{self, StatusCode};

use crate::{
    request,
    utils::{
        Utils,
        enums::{ApiError, BannerType},
        structs::{Connection, Promotion, Relationship, TokenInfo, UnauthorizedResponse},
    },
};

pub struct API<'a> {
    token: String,
    client: &'a reqwest::Client,
}

impl<'a> API<'a> {
    pub const API_URL: &'static str = "https://discord.com/api/v9";
    pub fn new(token: String, client: &'a reqwest::Client) -> API<'a> {
        API { token, client }
    }

    pub async fn get_me(&self) -> Result<TokenInfo, ApiError> {
        let response = request!(self.client, get, "/users/@me")?;

        let status = response.status();
        match status {
            StatusCode::OK => {
                let text = response.text().await?;
                let mut token_info: TokenInfo = serde_json::from_str(&text)?;

                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = self.token.clone();

                token_info.avatar = token_info
                    .avatar
                    .as_ref()
                    .map(|h| Utils::get_avatar(&token_info.id, h));

                token_info.banner = token_info
                    .banner
                    .as_ref()
                    .map(|h| Utils::get_banner(BannerType::User, &token_info.id, h));

                Ok(token_info)
            }
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse =
                    response
                        .json()
                        .await
                        .unwrap_or_else(|_err| UnauthorizedResponse {
                            code: 401,
                            message: "Unauthorized (parsing failed)".to_owned(),
                        });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            _ => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    pub async fn get_connections(&self) -> Result<Vec<Connection>, ApiError> {
        let response = request!(self.client, get, "/users/@me/connections")?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!(
                        "Warn: Failed to parse UNAUTHORIZED body (connections): {}",
                        e
                    );
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    pub async fn get_promotions(&self, locale: Option<&str>) -> Result<Vec<Promotion>, ApiError> {
        let response = request!(
            self.client,
            get,
            "/users/@me/outbound-promotions/codes",
            locale = locale.unwrap_or("en-US")
        )?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!(
                        "Warn: Failed to parse UNAUTHORIZED body (promotions): {}",
                        e
                    );
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_erre| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    pub async fn check_boosts(&self) -> Result<(), ApiError> {
        let response = request!(
            self.client,
            get,
            "/users/@me/guilds/premium/subscription-slots"
        )?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!("Warn: Failed to parse UNAUTHORIZED body (boosts): {}", e);
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    pub async fn get_relationships(&self) -> Result<Vec<Relationship>, ApiError> {
        let response = request!(self.client, get, "/users/@me/relationships")?;

        match response.status() {
            StatusCode::OK => {
                let response: Vec<Relationship> = response.json().await?;
                Ok(response)
            }
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!("Warn: Failed to parse UNAUTHORIZED body (boosts): {}", e);
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }
}
