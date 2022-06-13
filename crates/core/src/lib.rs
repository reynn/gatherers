pub mod directories;
pub mod downloaders;
pub mod gatherers;
pub mod http;
pub mod tasks;

pub type Result<T> = eyre::Result<T>;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
}
