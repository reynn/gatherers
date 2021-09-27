// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

pub mod directories;
pub mod downloaders;
pub mod errors;
pub mod gatherers;
pub mod http;
pub mod tasks;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;