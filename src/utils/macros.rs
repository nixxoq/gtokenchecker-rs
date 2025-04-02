/// Make a simple http request
///
/// USAGE:
///
///
/// `request!($self, $method, $path, $($key = $value),*)`: For making a request with one or more query parameters.
///     
/// * `$client`: reqwest::Client identifier
/// * `$method`: An identifier representing the HTTP method to use (e.g., `get`, `post`, `put`, `delete`).
/// * `$path`: An expression that evaluates to a string representing the path of the API endpoint (appended to `api::API_URL`).
/// * `$($key = $value),*`: A sequence of key-value pairs representing query parameters.
///     * `$key`: An identifier for the query parameter key.
///     * `$value`: An expression that evaluates to the value of the query parameter.
///
#[macro_export]
macro_rules! request {
    ($client:expr, $method:ident, $path:expr, $($key:ident = $value:expr),*) => {
        {
            let mut url = format!("{}{}", API::API_URL, $path);
            let mut params = Vec::new();
            $(
                params.push(format!("{}={}", stringify!($key), $value));
            )*
            if !params.is_empty() {
                url.push_str("?");
                url.push_str(&params.join("&"));
            }
            $client
                .$method(&url)
                .send()
                .await
                .unwrap()
        }
    };
    ($client:expr, $method:ident, $path:expr) => {
        $client
            .$method(format!("{}{}", API::API_URL, $path))
            .send()
            .await
            .unwrap()
    };
}
