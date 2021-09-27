// use std::fmt::;

pub use strum::{Display, EnumCount, EnumIter, EnumProperty};

#[derive(Debug, Display, EnumIter)]
pub enum GatherType {
    Posts,
    Messages,
    Bundles,
    Stories,
}
