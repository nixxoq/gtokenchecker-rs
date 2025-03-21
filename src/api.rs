use reqwest::{self, Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::utils::{CdnType, ImageType, StrOrInt, Utils};

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
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub banner_color: String,
    pub email: String,
    pub phone: Option<String>,
    pub mfa_enabled: bool,
    pub bio: Option<String>,

    // Skip these fields because discord api cannot return them and we will add these fields later during initialization process.
    #[serde(skip)]
    pub fullname: String,
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

        let username =
            Utils::get_string_value(dict, "username", Some("No username provided")).unwrap();
        let fullname = username.clone() + "#0000";
        let global_name =
            Utils::get_string_value(dict, "global_name", Some("No global username provided"))
                .unwrap();
        let discriminator = Utils::get_string_value(dict, "discriminator", Some("#0000")).unwrap();

        // TODO:
        // - implement a function that generates url for account/guild type for icons/banners by typing in arguments
        // for example gen_url("account", hash_value, "png");
        // explanation:
        // gen_url(type: &str, value: String (or &str, i will think about it), image_type: &str) -> String;
        // and yes... implement this on utils
        //

        // TODO: check if user have animated avatar and/or banner (a_XXXXXX startswith check)
        let avatar = Utils::get_string_value(dict, "avatar", None);
        let avatar_url = match avatar {
            Some(hash) => Utils::gen_url(CdnType::UserAvatar, &id, &hash, ImageType::Png),
            None => String::from("No avatar provided"),
        };

        let banner = Utils::get_string_value(dict, "banner", Some("No banner provided"));

        // TODO: check if user have animated avatar and/or banner (a_XXXXXX startswith check)
        let banner_url = match banner {
            Some(hash) => Utils::gen_url(CdnType::UserBanner, &id, &hash, ImageType::Png),
            None => String::from("No banner provided"),
        };

        let banner_color = Utils::get_string_value(dict, "banner_color", Some("#000000")).unwrap();

        let email = Utils::get_string_value(dict, "email", None).unwrap();
        let phone = Utils::get_string_value(dict, "phone", Some("No phone provided"));

        let mfa_enabled = match dict.get("mfa") {
            Some(StrOrInt::I32V(value)) => *value != 0,
            Some(StrOrInt::I64V(value)) => *value != 0,
            _ => false,
        };

        let bio = Utils::get_string_value(dict, "bio", Some("No bio provided"));
        let token = Utils::get_string_value(dict, "token", None)
            .unwrap_or(String::from("No token provided"));

        TokenInfo {
            id,
            username,
            global_name,
            discriminator,
            avatar: Some(avatar_url),
            banner: Some(banner_url),
            banner_color,
            email,
            phone,
            mfa_enabled,
            bio,
            fullname,
            token,
        }
    }

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
            self.bio.unwrap_or(String::from("No bio provided"))
        )
    }
}

pub struct API {}

impl API {
    pub const API_URL: &'static str = "https://discord.com/api/v9";

    // TODO: instead of initializing reqwest client every time, require in the get_me, and other functions in future, client parameter (&reqwest::Client)
    // UPD: don't forget to remove this once all function from original GTokenChecker will be migrated
    pub async fn get_me(
        client: &reqwest::Client,
        token: &str,
    ) -> Result<TokenInfo, UnauthorizedResponse> {
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

                // HINT Remove this line after researching results
                dbg!("{}", raw_json); // let it show some info for adding new features

                let mut token_info: TokenInfo = serde_json::from_str(&text).unwrap();
                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = String::from(token);

                // TODO: check if user have animated avatar and/or banner (a_XXXXXX startswith check)
                token_info.avatar = match token_info.avatar {
                    Some(hash) => Some(Utils::gen_url(
                        CdnType::UserAvatar,
                        &token_info.id,
                        &hash,
                        ImageType::Png,
                    )),
                    None => Some(String::from("No avatar available")),
                };
                token_info.banner = match token_info.banner {
                    Some(hash) => Some(Utils::gen_url(
                        CdnType::UserBanner,
                        &token_info.id,
                        &hash,
                        ImageType::Png,
                    )),
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

    pub async fn check(self) -> Result<TokenInfo, UnauthorizedResponse> {
        API::get_me(&self.client, &self.token).await
    }
}
