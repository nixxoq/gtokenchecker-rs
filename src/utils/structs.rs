use serde::{Deserialize, Serialize};

use crate::utils::Utils;

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
    pub rate_limited: bool,
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
            Utils::mask_last_part(self.token.as_str())
        } else {
            self.token
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
