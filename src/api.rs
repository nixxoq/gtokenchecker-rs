use reqwest::{
    self, StatusCode,
    header::{self, HeaderMap, HeaderValue},
};

use crate::{
    request,
    utils::{
        Utils,
        enums::{ApiError, BannerType},
        structs::{
            Connection, Promotion, Relationship, TokenInfo, TokenResult, UnauthorizedResponse,
        },
    },
};
use tokio::join;

pub struct API<'a> {
    token: String,
    client: &'a reqwest::Client,
}

impl<'a> API<'a> {
    pub const API_URL: &'static str = "https://discord.com/api/v9";
    pub fn new(token: String, client: &'a reqwest::Client) -> API<'a> {
        API { token, client }
    }

    async fn get_me(&self) -> Result<TokenInfo, ApiError> {
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

pub struct Checker {
    pub client: reqwest::Client,
    token: String,
}
impl Checker {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(token).expect("Invalid token format for header"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Checker {
            client: client.to_owned(),
            token: token.to_owned(),
        }
    }

    async fn process_token(
        &self,
        token_info: TokenInfo,
        api: &API<'_>,
    ) -> Result<TokenResult, ApiError> {
        let connections_future = api.get_connections();
        let promotions_future = api.get_promotions(Some(&token_info.locale));
        let boosts_future = api.check_boosts();
        let relationships_future = api.get_relationships();
        let mut rate_limited = false;

        let (connections_result, promotions_result, boosts_result, relationships_result) = join!(
            connections_future,
            promotions_future,
            boosts_future,
            relationships_future
        );

        let connections = match connections_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get connections: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        let promotions = match promotions_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get promotions: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        let relationships = match relationships_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get relationships: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        match boosts_result {
            Ok(_) => {}
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}...): Failed to check boosts: {}",
                    &self.token[..5],
                    e
                );
            }
        }

        Ok(TokenResult {
            main_info: token_info,
            connections,
            relationships,
            promotions,
            rate_limited,
        })
    }

    pub async fn check(self) -> Result<TokenResult, ApiError> {
        let api = API::new(self.token.clone(), &self.client);
        let token_info = api.get_me().await?;

        self.process_token(token_info, &api).await
    }
}
