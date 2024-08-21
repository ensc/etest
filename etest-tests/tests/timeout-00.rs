use std::thread::sleep;
use std::time::Duration;

use etest::prelude::*;

#[etest(timeout=1_000)]
fn test_0() {
}

#[should_panic]
#[etest(timeout=1_000)]
fn test_1() {
    sleep(Duration::from_millis(2_000));
}

#[should_panic]
#[etest(timeout=1_000)]
fn test_2() -> ! {
    #[allow(clippy::empty_loop)]
    loop {
    }
}
