
use reqwest::header;

// pub type Result<T> = std::result::Result<T, crate::errors::OpenLimitsError>;

trait Client {
    fn get_base_url(sandbox: bool) -> String;
    fn default_headers() -> header::HeaderMap<header::HeaderValue>;
}