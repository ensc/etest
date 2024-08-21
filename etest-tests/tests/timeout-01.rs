use std::time::Duration;

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

#[etest(timeout=2_000u64)]
fn test_3() {
}

#[etest(timeout=Timeout::new(Duration::from_millis(2_000)))]
fn test_4() {
}

#[etest(timeout=2_000)]
pub(self) fn test_5() {
}

#[etest(timeout=2_000)]
pub fn test_6() {
}
