use etest::prelude::*;

#[allow(dead_code)]
async fn wait() {
}

#[etest(timeout=1_000, consumes="A", test_fn=())]
async fn test_0() {
    wait().await
}

#[etest(consumes="A", test_fn=())]
async fn test_1() {
    wait().await
}

#[etest(consumes="A", test_fn=())]
async fn test_2() -> Result<(),()> {
    wait().await;

    Ok(())
}
