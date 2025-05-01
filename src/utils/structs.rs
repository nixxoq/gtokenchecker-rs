use nitro::Gift;
use serde::{Deserialize, Serialize};

pub mod connections;
pub mod guild;
pub mod nitro;
pub mod relationship;
#[path = "./structs/main_info.rs"]
pub mod token_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedResponse {
    pub code: i32,
    pub message: String,
}

pub struct TokenResult {
    pub main_info: token_info::TokenInfo,
    pub connections: Vec<connections::Connection>,
    pub relationships: Vec<relationship::Relationship>,
    pub promotions: Vec<nitro::Promotion>,
    pub rate_limited: bool,
    pub guilds: Vec<guild::Guild>,
    pub boosts: Vec<nitro::Boost>,
    pub nitro_info: Vec<nitro::Nitro>,
    pub nitro_credits: (usize, usize),
    pub gifts: Vec<Gift>,
}

impl TokenResult {
    pub fn show(self, mask_token: bool) {
        self.main_info.show(mask_token);
        println!("----------------------------- CONNECTIONS -----------------------------");
        if self.connections.is_empty() {
            println!("No connections available\n")
        } else {
            self.connections
                .iter()
                .enumerate()
                .for_each(|(index, connection)| connection.show(index, self.connections.len()))
        };
        println!("----------------------------- RELATIONSHIPS -----------------------------");
        if self.relationships.is_empty() {
            println!("No relationships available\n")
        } else {
            self.relationships
                .iter()
                .enumerate()
                .for_each(|(index, relationship)| {
                    relationship.show(index, self.relationships.len())
                })
        };
        println!("----------------------------- GUILDS -----------------------------");
        if self.guilds.is_empty() {
            println!("No guilds available\n")
        } else {
            self.guilds
                .iter()
                .enumerate()
                .for_each(|(index, guild)| guild.show(index, self.guilds.len()))
        };
        println!("----------------------------- BOOSTS -----------------------------");
        if self.boosts.is_empty() {
            println!("No boosts available\n")
        } else {
            self.boosts
                .iter()
                .enumerate()
                .for_each(|(index, boost)| boost.show(index, self.boosts.len()))
        };
        println!("----------------------------- PROMOTIONS -------------------------");
        if self.promotions.is_empty() {
            println!("No promotions available\n")
        } else {
            self.promotions
                .iter()
                .enumerate()
                .for_each(|(index, promotion)| promotion.show(index, self.connections.len()))
        };
        println!("-------------------------- NITRO & GIFTS -------------------------");
        println!("NITRO INFO:\n");
        if self.nitro_info.is_empty() {
            println!("No basic Nitro info available\n")
        } else {
            self.nitro_info
                .iter()
                .enumerate()
                .for_each(|(index, nitro)| nitro.show(index, self.nitro_info.len()))
        };
        println!(
            "
Nitro credits:

Classic: {}
Boost: {}
",
            self.nitro_credits.0, self.nitro_credits.1
        );
        println!("GIFTS INFO:\n");
        if self.gifts.is_empty() {
            println!("No gifts available\n")
        } else {
            self.gifts
                .iter()
                .enumerate()
                .for_each(|(index, gift)| gift.show(index, self.gifts.len()));
        }
    }
}
