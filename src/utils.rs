use std::collections::HashMap;

use chrono::DateTime;

use crate::api::StrOrInt;

pub fn get_account_creation(snowflake_id: i64, format: Option<&str>) -> String {
    let user_creation = ((snowflake_id >> 22) + 1420070400000) / 1000;
    let user_creation = DateTime::from_timestamp(user_creation, 0)
        .unwrap()
        .format(format.unwrap_or("%d.%m.%Y %H:%M:%S"))
        .to_string();

    user_creation
}

pub fn get_string_value(
    dict: &HashMap<String, StrOrInt>,
    key: &str,
    default_value: Option<&str>,
) -> Option<String> {
    match dict.get(key) {
        Some(StrOrInt::StrV(value)) => Some(value.clone()),
        _ => Some(default_value.unwrap_or("").to_string()),
    }
}
