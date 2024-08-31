#![allow(clippy::redundant_field_names)]

extern crate proc_macro;

mod defs {
    pub const CRATE_NAME: &str = "etest";
    pub const VARNAME_CURENT_TEST: &str = "etest_current_test";
}

mod errors;
mod utils;
mod config;
mod function;
mod macros;

use errors::Error;
use config::Config;
use function::Function;

pub use macros::etest;
