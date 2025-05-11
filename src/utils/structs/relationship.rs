use crate::utils::{Utils, constants::FRIEND_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: String,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
    pub public_flags: i128,
    pub username: String,
    pub discriminator: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub id: String,
    pub is_spam_request: bool,
    pub nickname: Option<String>,
    pub user: PublicUser,
    #[serde(rename = "since")]
    pub friendship_since: String,
    #[serde(rename = "type")]
    pub friend_type: i32,
}

impl Relationship {
    pub fn show(&self, index: usize, all_friends: usize) {
        let friend_type = FRIEND_TYPE
            .iter()
            .find_map(|(key, value)| (*key == self.friend_type).then_some(*value))
            .unwrap_or("Unknown type");

        let avatar = self
            .user
            .avatar
            .as_ref()
            .map(|hash| Utils::get_avatar(&self.id, hash))
            .unwrap_or("No avatar provided".to_owned());

        let friends_since = Utils::format_time(&self.friendship_since, None);
        let flags = Utils::get_user_flags(self.user.public_flags);

        let flags = if flags.is_empty() {
            "No public flags available".to_string()
        } else {
            flags.join(",")
        };

        println!(
            "Friend #{} of {}

ID: {}
Avatar: {}
Nickname: {}
Name#tag: {}#{}
Friend type: {}
Flags: {}
Friends since: {}
",
            index + 1,
            all_friends,
            self.id,
            avatar,
            self.nickname.as_ref().unwrap_or(&"No nickname".to_string()),
            self.user.username,
            self.user.discriminator,
            friend_type,
            flags,
            friends_since,
        )
    }
}
