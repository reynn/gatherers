pub mod directories;
pub mod downloaders;
pub mod gatherers;
pub mod http;
pub mod tasks;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
}
