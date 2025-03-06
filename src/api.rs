use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::utils::{StrOrInt, get_string_value};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub discriminator: String,
    #[serde(skip)]
    pub fullname: String,
    #[serde(skip)]
    pub legacy_username: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub mfa_enabled: bool,
    pub bio: Option<String>,
    #[serde(skip)]
    pub token: String,
}

impl TokenInfo {
    pub fn from_dict(dict: &HashMap<String, StrOrInt>) -> Self {
        let id = match dict.get("id") {
            Some(StrOrInt::I32V(value)) => value.to_string(),
            Some(StrOrInt::I64V(value)) => value.to_string(),
            Some(StrOrInt::I128V(value)) => value.to_string(),
            Some(StrOrInt::StrV(value)) => value.clone(),
            _ => String::new(),
        };

        let username = get_string_value(dict, "username", Some("No username provided")).unwrap();
        let fullname = username.clone() + "#0000";
        let global_name =
            get_string_value(dict, "global_name", Some("No global username provided")).unwrap();
        let discriminator = get_string_value(dict, "discriminator", Some("#0000")).unwrap();
        let legacy_username = get_string_value(
            dict,
            "legacy_username",
            Some("No legacy username available"),
        )
        .unwrap();

        let avatar = get_string_value(dict, "avatar", Some("No avatar provided"));
        let banner = get_string_value(dict, "banner", Some("No banner provided"));
        let email = get_string_value(dict, "email", None).unwrap();
        let phone = get_string_value(dict, "phone", Some("No phone provided"));

        let mfa_enabled = match dict.get("mfa") {
            Some(StrOrInt::I32V(value)) => *value != 0,
            Some(StrOrInt::I64V(value)) => *value != 0,
            _ => false,
        };

        let bio = get_string_value(dict, "bio", Some("No bio provided"));
        let token =
            get_string_value(dict, "token", None).unwrap_or(String::from("No token provided"));

        TokenInfo {
            id,
            username,
            global_name,
            discriminator,
            fullname,
            legacy_username,
            avatar,
            banner,
            email,
            phone,
            mfa_enabled,
            bio,
            token,
        }
    }

    pub fn show(self, mask_token: bool) {
        let token = if mask_token {
            let token_parts: Vec<&str> = self.token.split(".").collect();

            if token_parts.len() > 2 {
                let length = token_parts.last().unwrap().len();
                let last_part_masked = "*".repeat(length);
                format!("{}.{}.{}", token_parts[0], token_parts[1], last_part_masked)
            } else {
                self.token.clone()
            }
        } else {
            self.token.clone()
        };

        println!(
            "
Token: {}

ID: {}
username: {}
Full name: {}
Legacy name: {}
Avatar: {}
Banner: {}
E-mail: {}
Phone: {}
MFA: {}
Bio: {}",
            token,
            self.id,
            self.username,
            self.fullname,
            self.legacy_username,
            self.avatar.unwrap_or_default(),
            self.banner.unwrap_or(String::from("No banner provided")),
            self.email,
            self.phone.unwrap_or(String::from("No phone provided")),
            self.mfa_enabled,
            self.bio.unwrap()
        )
    }
}

pub struct API {}

impl API {
    pub const API_URL: &'static str = "https://discord.com/api/v9";

    pub async fn get_me(token: &str) -> Result<TokenInfo, UnauthorizedResponse> {
        // TODO: use serde and reqwest::Client
        let client = reqwest::Client::builder().build().unwrap();

        let response = client
            .get(format!("{}/users/@me", API::API_URL))
            .header("Authorization", token)
            .send()
            .await
            .unwrap();

        match response.status() {
            StatusCode::OK => {
                let text = response.text().await.unwrap();
                let raw_json: Value = serde_json::from_str(&text).unwrap();

                println!("{}", raw_json);

                let mut token_info: TokenInfo = serde_json::from_str(&text).unwrap();
                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = String::from(token);

                // workaround to parse legacy_username from older accounts if exists without panicing rust
                if let Some(legacy_username) =
                    raw_json.get("legacy_username").and_then(Value::as_str)
                {
                    token_info.legacy_username = legacy_username.to_string();
                } else {
                    token_info.legacy_username = String::from("No legacy username provided");
                }

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
}

pub struct Checker {
    token: String,
}

impl Checker {
    pub fn check() {}
}
