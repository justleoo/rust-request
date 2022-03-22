pub mod config {
    pub const _URL: &str = "https://httpbin.org/ip"; // for example: https://httpbin.org/ip
}

pub use self::config::_URL as url;
