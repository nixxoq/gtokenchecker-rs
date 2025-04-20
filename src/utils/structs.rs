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
        println!("----------------------------- PROMOTIONS -----------------------------");
        self.promotions
            .iter()
            .enumerate()
            .for_each(|(index, promotion)| promotion.show(index, self.connections.len()));
    }
}
