use reqwest::header::{self, HeaderMap, HeaderValue};
use tokio::join;

use crate::{
    api::API,
    utils::{
        Utils,
        enums::ApiError,
        structs::{TokenInfo, TokenResult},
    },
};

pub struct Checker {
    pub client: reqwest::Client,
    token: String,
}
impl Checker {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(token).expect("Invalid token format for header"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Checker {
            client: client.to_owned(),
            token: token.to_owned(),
        }
    }

    async fn process_token(
        &self,
        token_info: TokenInfo,
        api: &API<'_>,
    ) -> Result<TokenResult, ApiError> {
        let connections_future = api.get_connections();
        let promotions_future = api.get_promotions(Some(&token_info.locale));
        let boosts_future = api.check_boosts();
        let relationships_future = api.get_relationships();
        let mut rate_limited = false;

        let (connections_result, promotions_result, boosts_result, relationships_result) = join!(
            connections_future,
            promotions_future,
            boosts_future,
            relationships_future
        );

        let connections = match connections_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get connections: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        let promotions = match promotions_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get promotions: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        let relationships = match relationships_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}): Failed to get relationships: {}",
                    Utils::mask_last_part(&self.token),
                    e
                );
                Vec::new()
            }
        };

        match boosts_result {
            Ok(_) => {}
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}...): Failed to check boosts: {}",
                    &self.token[..5],
                    e
                );
            }
        }

        Ok(TokenResult {
            main_info: token_info,
            connections,
            relationships,
            promotions,
            rate_limited,
        })
    }

    pub async fn check(self) -> Result<TokenResult, ApiError> {
        let api = API::new(self.token.clone(), &self.client);
        let token_info = api.get_me().await?;

        self.process_token(token_info, &api).await
    }
}
