use reqwest::{
    self, StatusCode,
    header::{self, HeaderMap, HeaderValue},
};
use serde_json::Value;

use crate::{
    request,
    utils::{
        Utils,
        constants::USER_FLAGS,
        enums::{ApiError, BannerType},
        structs::{Connection, Promotion, TokenInfo, TokenResult, UnauthorizedResponse},
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
        API {
            token,
            client,
        }
    }

    // TODO: instead of initializing reqwest client every time, require in the get_me, and other functions in future, client parameter (&reqwest::Client)
    // UPD: don't forget to remove this once all function from original GTokenChecker will be migrated
    async fn get_me(&self) -> Result<TokenInfo, ApiError> {
        let response = request!(self.client, get, "/users/@me");

        let status = response.status();
        match status {
            StatusCode::OK => {
                let text = response.text().await?;
                let raw_json: Value = serde_json::from_str(&text)?;

                // HINT Remove this line after researching results
                dbg!("{}", raw_json); // let it show some info for adding new features

                let mut token_info: TokenInfo = serde_json::from_str(&text)?;
                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = self.token.clone();

                token_info.avatar = token_info
                    .avatar
                    .map(|hash| Utils::get_avatar(&token_info.id, &hash))
                    .or_else(|| Some(String::from("No avatar available")));

                token_info.banner = token_info
                    .banner
                    .map(|hash| Utils::get_banner(BannerType::User, &token_info.id, &hash))
                    .or_else(|| Some(String::from("No banner available")));

                Ok(token_info)
            }
            StatusCode::UNAUTHORIZED => {
                let unauthorized_response: UnauthorizedResponse = response
                    .json()
                    .await
                    .unwrap_or_else(|_| UnauthorizedResponse {
                        code: status.as_u16() as i32,
                        message: "Unauthorized (failed to parse error body)".to_string(),
                    });
                Err(ApiError::Unauthorized(unauthorized_response))
            }
            _ => {
                let error_body = response.text().await.unwrap_or_else(|_| {
                    status
                        .canonical_reason()
                        .unwrap_or("Unknown error")
                        .to_string()
                });
                Err(ApiError::UnexpectedStatus(status, error_body))
            }
        }
    }

    pub async fn get_connections(&self) -> Result<Vec<Connection>, UnauthorizedResponse> {
        let response = request!(self.client, get, "/users/@me/connections");

        match response.status() {
            StatusCode::OK => {
                let info: Vec<Connection> = response.json().await.unwrap();
                Ok(info)
            }
            StatusCode::UNAUTHORIZED => {
                let unauthorized_response: UnauthorizedResponse = response.json().await.unwrap();
                Err(unauthorized_response)
            }
            _ => {
                let unauthorized_response = UnauthorizedResponse {
                    code: response.status().as_u16() as i32,
                    message: format!("Unexpected status code: {}", response.status()),
                };
                Err(unauthorized_response)
            }
        }
    }

    pub async fn get_promotions(
        &self,
        locale: Option<&str>,
    ) -> Result<Vec<Promotion>, UnauthorizedResponse> {
        let response = request!(
            self.client,
            get,
            "/users/@me/outbound-promotions/codes",
            locale = locale.unwrap_or("us")
        );

        match response.status() {
            StatusCode::OK => {
                let info: Vec<Promotion> = response.json().await.unwrap();
                Ok(info)
            }
            StatusCode::UNAUTHORIZED => {
                let unauthorized_response: UnauthorizedResponse = response.json().await.unwrap();
                Err(unauthorized_response)
            }
            _ => {
                let unauthorized_response = UnauthorizedResponse {
                    code: response.status().as_u16() as i32,
                    message: format!("Unexpected status code: {}", response.status()),
                };
                Err(unauthorized_response)
            }
        }
    }

    pub async fn check_boosts(&self) -> Result<(), UnauthorizedResponse> {
        let response = self
            .client
            .get(format!(
                "{}/users/@me/guilds/premium/subscription-slots",
                API::API_URL
            ))
            .send()
            .await
            .unwrap();

        match response.status() {
            StatusCode::OK => {
                let raw_json: Value =
                    serde_json::from_str(&response.text().await.unwrap()).unwrap();
                println!("{raw_json}");
                Ok(())
            }
            StatusCode::UNAUTHORIZED => {
                let unauthorized_response: UnauthorizedResponse = response.json().await.unwrap();
                Err(unauthorized_response)
            }
            _ => {
                let unauthorized_response = UnauthorizedResponse {
                    code: response.status().as_u16() as i32,
                    message: format!("Unexpected status code: {}", response.status()),
                };
                Err(unauthorized_response)
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
        headers.insert(header::AUTHORIZATION, HeaderValue::from_str(token).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Checker {
            client: client.to_owned(),
            token: token.to_owned(),
        }
    }

    async fn process_token(
        &self,
        token: TokenInfo,
        api: &API<'_>,
    ) -> Result<TokenResult, ApiError> {
        let connections = api.get_connections();
        let promotions = api.get_promotions(Some(token.locale.as_ref()));
        let boosts = api.check_boosts();

        let (connections_result, promotions_result, _boosts) =
            join!(connections, promotions, boosts);

        Ok(TokenResult {
            main_info: token,
            connections: connections_result.unwrap(),
            promotions: promotions_result.unwrap(),
        })
    }

    pub async fn check(self) -> Result<TokenResult, ApiError> {
        let api = API::new(self.token.clone(), &self.client);

        let token_result = api.get_me().await;

        match token_result {
            Ok(token) => {
                let result = self.process_token(token, &api).await;
                Ok(result?)
            }
            Err(resp) => Err(resp),
        }
    }

    pub fn get_user_flags(self, public_flags: i128) -> Vec<String> {
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
}
