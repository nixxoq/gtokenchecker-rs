use reqwest::header::{self, HeaderMap, HeaderValue};
use tokio::join;

use crate::{
    api::API,
    utils::{
        enums::ApiError,
        structs::{token_info::TokenInfo, TokenResult},
        Utils,
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
    /// - `GET /users/@me/guilds`
    /// - `GET /users/@me/billing/subscriptions`
    /// - `GET /users/@me/applications/521842831262875670/entitlements?exclude_consumed=true` (available nitro credits)
    /// - `GET /users/@me/entitlements/gifts`
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

        let results = join!(
            api.get_connections(),
            api.get_promotions(Some(&token_info.locale)),
            api.get_relationships(),
            api.get_guilds(),
            api.check_boosts(),
            api.get_nitro_info(),
            api.check_nitro_credit(),
            api.get_gifts()
        );

        let connections = Utils::process_result(
            results.0,
            &self.token,
            "get connections",
            &mut rate_limited,
            Vec::new(),
        );
        let promotions = Utils::process_result(
            results.1,
            &self.token,
            "get promotions",
            &mut rate_limited,
            Vec::new(),
        );
        let relationships = Utils::process_result(
            results.2,
            &self.token,
            "get relationships",
            &mut rate_limited,
            Vec::new(),
        );
        let guilds = Utils::process_result(
            results.3,
            &self.token,
            "get guilds",
            &mut rate_limited,
            Vec::new(),
        );
        let boosts = Utils::process_result(
            results.4,
            &self.token,
            "check boosts",
            &mut rate_limited,
            Vec::new(),
        );
        let nitro = Utils::process_result(
            results.5,
            &self.token,
            "check nitro info",
            &mut rate_limited,
            Vec::new(),
        );
        let nitro_credits = Utils::process_result(
            results.6,
            &self.token,
            "check nitro credits",
            &mut rate_limited,
            (0, 0),
        );
        let gifts = Utils::process_result(
            results.7,
            &self.token,
            "get gifts",
            &mut rate_limited,
            Vec::new(),
        );

        Ok(TokenResult {
            main_info: token_info,
            connections,
            relationships,
            promotions,
            rate_limited,
            guilds,
            boosts,
            nitro_info: nitro,
            nitro_credits,
            gifts,
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
