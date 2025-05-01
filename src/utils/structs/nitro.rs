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
pub struct Nitro {
    pub id: String,
    #[serde(rename = "type")]
    pub nitro_type: i32,
    pub created_at: String,
    pub canceled_at: Option<String>,
    #[serde(rename = "current_period_start")]
    pub start_time: Option<String>,
    #[serde(rename = "current_period_end")]
    pub end_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gift {
    pub subscription_plan: Option<SubscriptionPlan>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionPlan {
    // I don't know all structure of SubscriptionPlan, so I copied from original GTokenChecker
    pub name: String,
}

impl Gift {
    pub fn show(&self, index: usize, all_gifts: usize) {
        let subscription_name = &self.subscription_plan.as_ref().unwrap().name;

        println!(
            "
Gift info #{} of {}

Subscription name: {}",
            index + 1,
            all_gifts,
            subscription_name
        )
    }
}

impl Nitro {
    pub fn show(&self, index: usize, all_nitro: usize) {
        let nitro_type = if self.nitro_type == 1 {
            "nitro"
        } else if self.nitro_type == 2 {
            "nitro classic"
        } else if self.nitro_type == 3 {
            "nitro basic"
        } else {
            "unknown type"
        };

        let start_date = match &self.start_time {
            Some(time) => Utils::format_time(&time, None),
            _ => "no start time provided".to_owned(),
        };

        let end_date = match &self.end_time {
            Some(time) => Utils::format_time(&time, None),
            _ => "no end time provided".to_owned(),
        };

        let cancel_date = match &self.canceled_at {
            Some(time) => Utils::format_time(&time, None),
            _ => "no cancellation time provided".to_owned(),
        };

        println!(
            "
Nitro info #{} of {}

Id: {}
Start time: {}
End time: {}
Cancelation time: {}
Nitro type: {}
",
            index + 1,
            all_nitro,
            self.id,
            start_date,
            end_date,
            cancel_date,
            nitro_type,
        )
    }
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
