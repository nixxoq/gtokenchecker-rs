use crate::utils::{Utils, enums::BannerType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[serde(rename = "owner")]
    pub is_owner: bool,
    pub permissions: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    #[serde(rename = "approximate_member_count")]
    pub member_count: i128,
}

impl Guild {
    pub fn show(&self, index: usize, all_guilds: usize) {
        let permissions = Utils::get_user_permissions(&self.permissions).join(", ");
        let icon = self
            .icon
            .as_ref()
            .map(|hash| Utils::get_avatar(&self.id, hash))
            .unwrap_or("No icon provided".to_owned());

        let banner = self
            .banner
            .as_ref()
            .map(|hash| Utils::get_banner(BannerType::Guild, &self.id, hash))
            .unwrap_or("No banner provided".to_owned());

        println!(
            "Guild #{} of {}
ID: {}
Name: {}
Is user owner: {}
User permissions: {}
Icon: {}
Banner: {}
Member count: {}
",
            index + 1,
            all_guilds,
            self.id,
            self.name,
            self.is_owner,
            permissions,
            icon,
            banner,
            self.member_count
        )
    }
}
