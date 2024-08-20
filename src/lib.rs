#![allow(clippy::redundant_field_names)]

// declares macros for use in crate; must be on top of file
mod logging;

pub use etest_derive::etest;

mod resource;
mod location;
mod default_return;
mod helpers;

pub use location::Location;
pub use resource::{ ResourceBuilder, ResourceId, RESOURCES };
pub use default_return::DefaultReturn;

pub use helpers::*;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use crate::DefaultReturn;
}
