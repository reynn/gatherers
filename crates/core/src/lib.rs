// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![warn(clippy::all)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_macros, unused_imports, unused_variables)
)]

pub mod directories;
pub mod downloaders;
pub mod gatherers;
pub mod http;
pub mod tasks;
#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
}
