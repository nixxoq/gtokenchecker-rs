use api::TokenInfo;
use utils::get_account_creation;

pub mod api;
pub mod utils;

#[tokio::main]
async fn main() {
    get_account_creation(935942230634532884, None);

    // use api::StrOrInt::*;

    let result = api::API::get_me("token").await;

    match result {
        Ok(token_info) => {
            token_info.show(true); // mask_token -> bool
        }
        Err(err) => {
            eprintln!("Error: {}", err.message)
        }
    }
}
