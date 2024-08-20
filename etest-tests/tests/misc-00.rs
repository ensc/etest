#![allow(clippy::no_effect)]

//! Tests arguments in test functions.
//!
//! There is no direct use atm, but perhaps later...

use etest::prelude::*;

#[etest(test_fn=())]
fn test_0(_a: u32) {
    23;
}

#[etest(timeout=1, test_fn=())]
fn test_1(a: u32) {
    let _ = a == 23;
}

// TODO: disabled for now until 'async' closures are fully supported
#[cfg(any())]
#[etest(timeout=1, test_fn=())]
async fn test_2(a: u32) {
    a == 23;
}
