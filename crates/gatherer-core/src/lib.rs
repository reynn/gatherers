pub mod config;
pub mod directories;
pub mod downloaders;
pub mod errors;
pub mod gatherers;
pub mod http;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;