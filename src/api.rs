use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{BannerType, USER_FLAGS, Utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub name: String,
    pub visibility: u8,
    pub verified: bool,
    pub revoked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Promotion {
    outbound_title: String,
    start_time: String,
    end_date: String,
    #[serde(rename = "outbound_redemption_page_link")]
    link: String,
    code: String,
}

pub struct TokenResult {
    pub main_info: TokenInfo,
    pub connections: Vec<Connection>,
    pub promotions: Vec<Promotion>,
}

impl TokenResult {
    pub fn show(self, mask_token: bool) {
        self.main_info.show(mask_token);
        self.connections
            .iter()
            .for_each(|connection| println!("{}: {}", connection.connection_type, connection.name));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenInfo {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub banner_color: String,
    pub email: String,
    pub locale: String,
    // pub pronouns: Option<String>,
    pub phone: Option<String>,
    pub mfa_enabled: bool,
    pub bio: Option<String>,
    pub public_flags: i128,

    // Skip these fields because discord api cannot return them and we will add these fields later during initialization process.
    #[serde(skip)]
    pub fullname: String,
    #[serde(skip)]
    pub token: String,
}

impl TokenInfo {
    pub fn show(self, mask_token: bool) {
        let token = if mask_token {
            let mut parts: Vec<_> = self.token.split('.').map(|part| part.to_string()).collect();
            parts.last_mut().map(|last| *last = "*".repeat(last.len()));
            parts.join(".")
        } else {
            self.token.clone()
        };

        println!(
            "
Token: {}

ID: {}
username: {}
Full name: {}
Avatar: {}
Banner: {}
Banner color: {}
E-mail: {}
Phone: {}
MFA: {}
Bio: {}",
            // pronouns: {}
            token,
            self.id,
            self.username,
            self.fullname,
            self.avatar.unwrap_or(String::from("No banner provided")),
            self.banner.unwrap_or(String::from("No banner provided")),
            self.banner_color,
            self.email,
            self.phone.unwrap_or(String::from("No phone provided")),
            self.mfa_enabled,
            self.bio.unwrap_or(String::from("No bio provided")),
            // self.pronouns
            //     .unwrap_or(String::from("No pronouns available"))
        )
    }
}

#[derive(Clone)]
pub struct API<'a> {
    token: String,
    client: &'a reqwest::Client,
}

impl API<'_> {
    pub const API_URL: &'static str = "https://discord.com/api/v9";

    pub fn new(token: String, client: &reqwest::Client) -> API {
        API {
            token: token,
            client: client,
        }
    }

    // TODO: instead of initializing reqwest client every time, require in the get_me, and other functions in future, client parameter (&reqwest::Client)
    // UPD: don't forget to remove this once all function from original GTokenChecker will be migrated
    async fn get_me(
        self,
        // client: &reqwest::Client,
        // token: &str,
    ) -> Result<TokenInfo, UnauthorizedResponse> {
        let response = self
            .client
            .get(format!("{}/users/@me", API::API_URL))
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap();

        match response.status() {
            StatusCode::OK => {
                let text = response.text().await.unwrap();
                let raw_json: Value = serde_json::from_str(&text).unwrap();

                // HINT Remove this line after researching results
                dbg!("{}", raw_json); // let it show some info for adding new features

                let mut token_info: TokenInfo = serde_json::from_str(&text).unwrap();
                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = self.token;

                token_info.avatar = match token_info.avatar {
                    Some(hash) => Some(Utils::get_avatar(&token_info.id, &hash)),
                    None => Some(String::from("No avatar available")),
                };
                token_info.banner = match token_info.banner {
                    Some(hash) => Some(Utils::get_banner(BannerType::User, &token_info.id, &hash)),
                    None => Some(String::from("No banner provided")),
                };

                Ok(token_info)
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

    pub async fn get_connections(self) -> Result<Vec<Connection>, UnauthorizedResponse> {
        let response = self
            .client
            .get(format!("{}/users/@me/connections", API::API_URL))
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap();

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
        self,
        locale: Option<String>,
    ) -> Result<Vec<Promotion>, UnauthorizedResponse> {
        let response = self
            .client
            .get(format!(
                "{}/users/@me/outbound-promotions/codes?locale={}",
                API::API_URL,
                locale.unwrap_or(String::from("us"))
            ))
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap();

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
}

pub struct Checker {
    pub client: reqwest::Client,
    token: String,
}

impl Checker {
    pub fn new(token: &str) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Checker {
            client: client,
            token: String::from(token),
        }
    }

    async fn process_token(self, token: TokenInfo, api: API<'_>) -> TokenResult {
        let connections = api.clone().get_connections().await.unwrap();
        let promotions = api
            .clone()
            .get_promotions(Some(token.clone().locale))
            .await
            .unwrap();

        TokenResult {
            main_info: token,
            connections: connections,
            promotions: promotions,
        }
    }

    pub async fn check(self) -> Result<TokenResult, UnauthorizedResponse> {
        let client = self.client.clone();
        let api = API::new(self.token.clone(), &client);
        let token_result = api.clone().get_me().await;

        match token_result {
            Ok(token) => Ok(self.process_token(token, api.clone()).await),
            Err(resp) => return Err(resp),
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
