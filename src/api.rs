use reqwest::{self, StatusCode};

use crate::{
    request,
    utils::{
        Utils,
        enums::{ApiError, BannerType},
        structs::{Connection, Guild, Promotion, Relationship, TokenInfo, UnauthorizedResponse},
    },
};

pub struct API<'a> {
    token: String,
    client: &'a reqwest::Client,
}

impl<'a> API<'a> {
    pub const API_URL: &'static str = "https://discord.com/api/v9";

    /// Creates a new `API` instance with the given token and client.
    ///
    /// # Parameters
    ///
    /// * `token`: The Discord token to use for API requests.
    /// * `client`: The HTTP client to use for API requests.
    ///
    /// # Returns
    ///
    /// A new `API` instance.
    pub fn new(token: String, client: &'a reqwest::Client) -> API<'a> {
        API { token, client }
    }

    /// Retrieves the user's token information from the Discord API.
    ///
    /// This function sends a `GET /users/@me` request to the Discord API to fetch
    /// the user's token information. On a successful response, it populates the
    /// `TokenInfo` struct with the user's details, including their full name,
    /// token, avatar, and banner URLs.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenInfo)`: Contains the user's token information if the request is
    ///   successful.
    /// * `Err(ApiError)`: An error if the request fails, which could be due to
    ///   unauthorized access, rate limiting, or unexpected status codes.
    ///
    /// # Errors
    ///
    /// This function returns an `ApiError` if the request is unauthorized, rate limited,
    /// or if an unexpected status code is received. The error details will include the
    /// response status and any relevant error messages.
    pub async fn get_me(&self) -> Result<TokenInfo, ApiError> {
        let response = request!(self.client, get, "/users/@me")?;

        let status = response.status();
        match status {
            StatusCode::OK => {
                let text = response.text().await?;
                let mut token_info: TokenInfo = serde_json::from_str(&text)?;

                token_info.fullname =
                    format!("{}#{}", token_info.username, token_info.discriminator);
                token_info.token = self.token.clone();

                token_info.avatar = token_info
                    .avatar
                    .as_ref()
                    .map(|h| Utils::get_avatar(&token_info.id, h));

                token_info.banner = token_info
                    .banner
                    .as_ref()
                    .map(|h| Utils::get_banner(BannerType::User, &token_info.id, h));

                Ok(token_info)
            }
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse =
                    response
                        .json()
                        .await
                        .unwrap_or_else(|_err| UnauthorizedResponse {
                            code: 401,
                            message: "Unauthorized (parsing failed)".to_owned(),
                        });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            _ => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    /// Fetches the user's connections from the API.
    ///
    /// This asynchronous function makes a GET request to the `/users/@me/connections` endpoint
    /// to retrieve a list of connections associated with the user. Connections can include
    /// linked accounts from other platforms such as Xbox, Steam, etc.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Connection>)`: A vector of `Connection` objects if the request is successful.
    /// * `Err(ApiError)`: An error if the request fails due to unauthorized access, rate limiting, or
    ///   an unexpected status code.
    ///
    /// # Errors
    ///
    /// * `ApiError::Unauthorized`: Returned if the request fails with a 401 Unauthorized status.
    /// * `ApiError::RateLimited`: Returned if the request fails due to rate limiting.
    /// * `ApiError::UnexpectedStatus`: Returned for any unexpected status codes.
    pub async fn get_connections(&self) -> Result<Vec<Connection>, ApiError> {
        let response = request!(self.client, get, "/users/@me/connections")?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!(
                        "Warn: Failed to parse UNAUTHORIZED body (connections): {}",
                        e
                    );
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    /// Fetches outbound promotions available to the user.
    ///
    /// This asynchronous function makes a GET request to the `/users/@me/outbound-promotions/codes`
    /// endpoint to retrieve a list of promotions the user can avail. The request can be localized
    /// by providing an optional `locale` parameter, defaulting to "en-US" if not specified.
    ///
    /// # Arguments
    ///
    /// * `locale`: An optional string slice representing the desired locale for the promotions.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Promotion>)`: A vector of `Promotion` objects if the request is successful.
    /// * `Err(ApiError)`: An error if the request fails due to unauthorized access, rate limiting,
    ///   or an unexpected status code.
    ///
    /// # Errors
    ///
    /// * `ApiError::Unauthorized`: Returned if the request fails with a 401 Unauthorized status.
    /// * `ApiError::RateLimited`: Returned if the request fails due to rate limiting.
    /// * `ApiError::UnexpectedStatus`: Returned for any unexpected status codes.
    pub async fn get_promotions(&self, locale: Option<&str>) -> Result<Vec<Promotion>, ApiError> {
        let response = request!(
            self.client,
            get,
            "/users/@me/outbound-promotions/codes",
            locale = locale.unwrap_or("en-US")
        )?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!(
                        "Warn: Failed to parse UNAUTHORIZED body (promotions): {}",
                        e
                    );
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_erre| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    /// Checks if the user has any active boosts.
    ///
    /// This function sends a GET request to the `/users/@me/guilds/premium/subscription-slots` endpoint
    /// to check if the user has any active boosts. If the request is successful, it returns an empty
    /// `Result`. If the request fails, it returns an `ApiError` containing the status code and any
    /// relevant error messages.
    ///
    /// # Returns
    ///
    /// * `Ok(())`: If the request is successful.
    /// * `Err(ApiError)`: If the request fails.
    ///
    /// # Errors
    ///
    /// * `ApiError::Unauthorized`: Returned if the request fails with a 401 Unauthorized status.
    /// * `ApiError::RateLimited`: Returned if the request fails due to rate limiting.
    /// * `ApiError::UnexpectedStatus`: Returned for any unexpected status codes.
    pub async fn check_boosts(&self) -> Result<(), ApiError> {
        let response = request!(
            self.client,
            get,
            "/users/@me/guilds/premium/subscription-slots"
        )?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!("Warn: Failed to parse UNAUTHORIZED body (boosts): {}", e);
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    /// Retrieves the user's friendships from the Discord API.
    ///
    /// This asynchronous function makes a GET request to the `/users/@me/relationships` endpoint
    /// to fetch the user's friendships. If the request is successful, it returns a vector of
    /// `Relationship` objects. If the request fails, it returns an `ApiError` containing the status
    /// code and any relevant error messages.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Relationship>)`: A vector of `Relationship` objects if the request is successful.
    /// * `Err(ApiError)`: If the request fails due to unauthorized access, rate limiting, or an
    ///   unexpected status code.
    ///
    /// # Errors
    ///
    /// * `ApiError::Unauthorized`: Returned if the request fails with a 401 Unauthorized status.
    /// * `ApiError::RateLimited`: Returned if the request fails due to rate limiting.
    /// * `ApiError::UnexpectedStatus`: Returned for any unexpected status codes.
    pub async fn get_relationships(&self) -> Result<Vec<Relationship>, ApiError> {
        let response = request!(self.client, get, "/users/@me/relationships")?;

        match response.status() {
            StatusCode::OK => {
                let response: Vec<Relationship> = response.json().await?;
                Ok(response)
            }
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!(
                        "Warn: Failed to parse UNAUTHORIZED body (relationships): {}",
                        e
                    );
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }

    pub async fn get_guilds(&self) -> Result<Vec<Guild>, ApiError> {
        let response = request!(self.client, get, "/users/@me/guilds?with_counts=true")?;

        match response.status() {
            StatusCode::OK => {
                let guilds: Vec<Guild> = response.json().await?;
                Ok(guilds)
            }
            StatusCode::UNAUTHORIZED => {
                let resp: UnauthorizedResponse = response.json().await.unwrap_or_else(|e| {
                    eprintln!("Warn: Failed to parse UNAUTHORIZED body (guilds): {}", e);
                    UnauthorizedResponse {
                        code: 401,
                        message: "Unauthorized (parsing failed)".into(),
                    }
                });
                Err(ApiError::Unauthorized(resp))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let resp_opt: Option<UnauthorizedResponse> = response.json().await.ok();
                Err(ApiError::RateLimited(resp_opt))
            }
            status => {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_err| format!("Status: {}", status));
                Err(ApiError::UnexpectedStatus(status, body))
            }
        }
    }
}
