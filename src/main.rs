use std::collections::HashMap;

use api::{StrOrInt, TokenInfo};
use utils::get_account_creation;

pub mod api;
pub mod utils;

fn main() {
    get_account_creation(935942230634532884, None);

    use api::StrOrInt::*;
    let mut test_dict: HashMap<String, StrOrInt> = HashMap::new();
    test_dict.insert("id".to_string(), I64V(12345));
    test_dict.insert("username".to_string(), StrV("testuser".to_string()));
    test_dict.insert("email".to_string(), StrV("test@example.com".to_string()));
    test_dict.insert("mfa".to_string(), I32V(1));
    test_dict.insert("fullname".to_string(), StrV("Test User".to_string()));

    let result = TokenInfo::from_dict(&test_dict);

    result.show()
    // println!("TokenInfo id: {}", result.username);
}
