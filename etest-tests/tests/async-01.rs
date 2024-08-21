#![cfg(feature = "tokio")]

use etest::prelude::*;

#[allow(dead_code)]
async fn wait() {
}

#[etest(timeout=1_000, consumes="A")]
async fn test_0() {
    wait().await
}

#[etest(timeout=1_000, consumes="A")]
pub async fn test_1() {
    wait().await
}

#[etest(timeout=1_000, consumes="A")]
pub(self) async fn test_1() {
    wait().await
}
