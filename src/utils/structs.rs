use serde::{Deserialize, Serialize};

use crate::utils::{Utils, constants::FRIEND_TYPE, enums::BannerType};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PremiumGuildSubscription {
    pub id: String,
    pub user_id: String,
    pub guild_id: String,
    pub ended: bool,
    pub pause_ends_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boost {
    pub id: String,
    pub subscription_id: String,
    pub premium_guild_subscription: Option<PremiumGuildSubscription>,
    pub canceled: bool,
    #[serde(rename = "cooldown_ends_at")]
    pub ends: String,
}

impl Boost {
    pub fn show(&self, index: usize, all_boosts: usize) {
        let end_date = Utils::format_time(&self.ends, None);
        let canceled = if self.canceled { "Yes" } else { "No" };
        let is_used = if !self.premium_guild_subscription.is_none() {
            "Yes"
        } else {
            "No"
        };
        let guild_id: &String = if let Some(guild) = &self.premium_guild_subscription {
            // TODO: implement config options for gtokenchecker with option to show guild name instead of ID
            // this option will use /@users/me/guilds/{guild.id}/basic endpoint additionally
            &guild.guild_id
        } else {
            &"No guild (unused)".to_owned()
        };

        println!(
            "
Boost #{} of {}

Is used: {}
Subscription ID: {}
Guild ID: {}
Canceled: {}
Ends: {}
",
            index + 1,
            all_boosts,
            is_used,
            self.subscription_id,
            guild_id,
            canceled,
            end_date,
        )
    }
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

impl Connection {
    pub fn show(&self, index: usize, all_connections: usize) {
        println!(
            "
Connection #{} of {}

Connection type: {}
Name: {}
Visible: {}
Verified: {}
Revoked: {}
",
            index,
            all_connections,
            self.connection_type,
            self.name,
            self.visibility != 0,
            self.verified,
            self.revoked
        )
    }
}

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
    // pub friendship_since: Option<String>,
    #[serde(rename = "type")]
    pub friend_type: i32,
}

impl Relationship {
    pub fn show(&self, index: usize, all_friends: usize) {
        let friend_type = FRIEND_TYPE
            .iter()
            .find(|&&(key, _value)| key == self.friend_type)
            .map(|&(_k, v)| v)
            .unwrap_or("Unknown type");

        let avatar = self
            .user
            .avatar
            .as_ref()
            .map(|hash| Utils::get_avatar(&self.id, hash))
            .unwrap_or("No avatar provided".to_owned());

        let friends_since = Utils::format_time(&self.friendship_since, None);
        let flags = Utils::get_user_flags(self.user.public_flags);

        let flags = match flags.is_empty() {
            true => "No public flags available".to_string(),
            false => flags.join(","),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Promotion {
    outbound_title: String,
    start_time: String,
    end_date: String,
    #[serde(rename = "outbound_redemption_page_link")]
    link: String,
    code: String,
}

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

// Maybe should I create trait?

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

pub struct TokenResult {
    pub main_info: TokenInfo,
    pub connections: Vec<Connection>,
    pub relationships: Vec<Relationship>,
    pub promotions: Vec<Promotion>,
    pub rate_limited: bool,
    pub guilds: Vec<Guild>,
    pub boosts: Vec<Boost>,
}

impl TokenResult {
    pub fn show(self, mask_token: bool) {
        self.main_info.show(mask_token);
        println!("----------------------------- CONNECTIONS -----------------------------");
        self.connections
            .iter()
            .enumerate()
            .for_each(|(index, connection)| connection.show(index, self.connections.len()));
        println!("----------------------------- RELATIONSHIPS -----------------------------");
        self.relationships
            .iter()
            .enumerate()
            .for_each(|(index, relationship)| relationship.show(index, self.relationships.len()));
        println!("----------------------------- GUILDS -----------------------------");
        self.guilds
            .iter()
            .enumerate()
            .for_each(|(index, guild)| guild.show(index, self.guilds.len()));
        println!("----------------------------- BOOSTS -----------------------------");
        self.boosts
            .iter()
            .enumerate()
            .for_each(|(index, boost)| boost.show(index, self.boosts.len()));
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
