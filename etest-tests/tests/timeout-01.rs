use etest::prelude::*;

#[etest(timeout=1_000)]
fn test_0() {
}

#[etest(timeout=std::time::Duration::from_millis(1_000))]
fn test_1() {
}


fn timeout() -> u32 {
    23
}

#[etest(timeout=timeout())]
fn test_2() {
}
