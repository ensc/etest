//! Test unwinding of resource allocation

use std::thread::sleep;
use std::time::Duration;

use etest::prelude::*;

#[should_panic]
#[etest(timeout=1_000, consumes="A")]
fn test_0() {
    sleep(Duration::from_millis(2_000));
}

#[should_panic]
#[etest(timeout=1_000, consumes="A")]
fn test_1() {
    sleep(Duration::from_millis(2_000));
}

#[should_panic]
#[etest(timeout=1_000, consumes="A")]
fn test_2() {
    panic!();
}

#[etest(timeout=1_000, consumes="A")]
fn test_3() {
}
