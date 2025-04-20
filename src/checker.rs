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
    /// Creates a new `Checker` instance with the given token.
    ///
    /// # Parameters
    ///
    /// * `token`: The Discord token to use for checking.
    ///
    /// # Returns
    ///
    /// A new `Checker` instance.
    ///
    /// # Errors
    ///
    /// This function will panic if the token cannot be inserted into the headers
    /// or if the client cannot be built.
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

    /// Processes a given token and returns the result of all API calls.
    ///
    /// This function calls the following API endpoints in parallel:
    ///
    /// - `GET /users/@me/connections`
    /// - `GET /users/@me/guilds/premium/subscription-slots`
    /// - `GET /users/@me/guilds/premium/subscriptions`
    /// - `GET /users/@me/relationships`
    ///
    /// If any of the API calls return a rate limit error, this function will return a `TokenResult`
    /// with `rate_limited` set to `true`. If any of the API calls fail with an error that is not
    /// a rate limit error, this function will log the error with the `Warn` level and continue
    /// with the next API call.
    ///
    /// # Errors
    ///
    /// If any of the API calls fail with an error that is not a rate limit error, this function
    /// will log the error with the `Warn` level and continue with the next API call. If any of
    /// the API calls return a rate limit error, this function will return a `TokenResult` with
    /// `rate_limited` set to `true`.
    ///
    /// # Panics
    ///
    /// This function will panic if the API calls fail with an error that is not a rate limit
    /// error.
    async fn process_token(
        &self,
        token_info: TokenInfo,
        api: &API<'_>,
    ) -> Result<TokenResult, ApiError> {
        let mut rate_limited = false;

        let (
            connections_result,
            promotions_result,
            boosts_result,
            relationships_result,
            guilds_result,
        ) = join!(
            api.get_connections(),
            api.get_promotions(Some(&token_info.locale)),
            api.check_boosts(),
            api.get_relationships(),
            api.get_guilds()
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

        let guilds = match guilds_result {
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

        let boosts = match boosts_result {
            Ok(data) => data,
            Err(ApiError::RateLimited(_)) => {
                rate_limited = true;
                Vec::new()
            }
            Err(e) => {
                eprintln!(
                    " Warn (token: {}...): Failed to check boosts: {}",
                    &self.token[..5],
                    e
                );
                Vec::new()
            }
        };

        Ok(TokenResult {
            main_info: token_info,
            connections,
            relationships,
            promotions,
            rate_limited,
            guilds,
            boosts,
        })
    }

    /// Checks the token and returns the result of all API calls.
    ///
    /// This function initializes the API client and retrieves the user's token information
    /// using the `get_me` API call. It then processes the token by calling the `process_token`
    /// method, which executes several API endpoints in parallel to gather data related to the
    /// token.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenResult)`: A `TokenResult` containing the token information and results
    ///   from the API calls, if successful.
    /// * `Err(ApiError)`: An error if the initial `get_me` API call or any subsequent
    ///   API calls fail.
    pub async fn check(self) -> Result<TokenResult, ApiError> {
        let api = API::new(self.token.clone(), &self.client);
        let token_info = api.get_me().await?;

        self.process_token(token_info, &api).await
    }
}
