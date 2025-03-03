use std::collections::HashMap;

use crate::utils::get_string_value;

pub enum StrOrInt {
    StrV(String),
    I32V(i32),
    I64V(i64),
    I128V(i128),
}

pub struct TokenInfo {
    pub id: i128,
    pub username: String,
    pub fullname: String,
    pub legacy_username: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub mfa: bool,
    pub bio: Option<String>,
    token: String,
}

impl TokenInfo {
    pub fn from_dict(dict: &HashMap<String, StrOrInt>) -> Self {
        let id = match dict.get("id") {
            Some(StrOrInt::I32V(value)) => *value as i128,
            Some(StrOrInt::I64V(value)) => *value as i128,
            Some(StrOrInt::StrV(value)) => value.parse::<i128>().unwrap_or(0),
            Some(StrOrInt::I128V(value)) => *value,
            _ => 0,
        };

        let username = get_string_value(dict, "username", Some("No username provided")).unwrap();
        let fullname = username.clone() + "#0000";
        let legacy_username = get_string_value(
            dict,
            "legacy_username",
            Some("No legacy username available"),
        );
        // let fullname = get_string_value(dict, "fullname");
        let avatar = get_string_value(dict, "avatar", Some("No avatar provided"));
        let banner = get_string_value(dict, "banner", Some("No banner provided"));
        let email = get_string_value(dict, "email", None).unwrap();
        let phone = get_string_value(dict, "phone", Some("No phone provided"));

        let mfa = match dict.get("mfa") {
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
            fullname,
            legacy_username,
            avatar,
            banner,
            email,
            phone,
            mfa,
            bio,
            token,
        }
    }

    pub fn show(self) {
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
            self.token,
            self.id,
            self.username,
            self.fullname,
            self.legacy_username.unwrap(),
            self.avatar.unwrap(),
            self.banner.unwrap(),
            self.email,
            self.phone.unwrap(),
            self.mfa,
            self.bio.unwrap()
        )
    }
}

pub struct Checker {
    token: String,
}

impl Checker {
    pub fn check() {}
}
