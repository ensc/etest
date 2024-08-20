#![cfg(feature = "tokio")]

use etest::prelude::*;

#[allow(dead_code)]
async fn wait() {
}

#[etest(timeout=1_000, consumes="A")]
async fn test_0() {
    wait().await
}
