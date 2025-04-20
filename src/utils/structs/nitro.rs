use crate::utils::Utils;
use serde::{Deserialize, Serialize};

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

impl Promotion {
    pub fn show(&self, index: usize, all_promotions: usize) {
        let start_date = Utils::format_time(&self.start_time, None);
        let end_date = Utils::format_time(&self.end_date, None);
        println!(
            "
Promotion #{} of {}

Title: {}
Start date: {}
End date: {}
Link: {}
Code: {}
",
            index + 1,
            all_promotions,
            self.outbound_title,
            start_date,
            end_date,
            self.link,
            self.code
        )
    }
}
