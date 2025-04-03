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
    ($client:expr, $method:ident, $path:expr, $($key:ident = $value:expr),* $(,)?) => {
        {
            let query_string: String = {
                let qs: Vec<String> = vec![
                    $(format!("{}={}", stringify!($key), $value)),*
                ];
                qs.join("&")
            };

            let url = format!("{}{}?{}", API::API_URL, $path, query_string);
            $client
                .$method(&url)
                .send()
                .await
        }
    };
    ($client:expr, $method:ident, $path:expr) => {
        $client
            .$method(format!("{}{}", API::API_URL, $path))
            .send()
            .await
    };
}
