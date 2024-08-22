//! Create a deadlock when consuming a resource and check that timeout is
//! triggered.
//!
//! Do this also when skipping the inner test which should prevent the
//! deadlock.

use etest::prelude::*;

fn check_a(a: u32) -> bool {
    a != 23
}


#[etest(timeout=2_000, skip=check_a(a), consumes="A", test_fn=())]
fn test_inner_0(a: u32) {
}

#[should_panic]
#[etest(timeout=2_000, consumes="A")]
fn test_outer_0() {
    test_inner_0(23);
}



#[etest(timeout=2_000, skip=check_a(a), consumes="B", test_fn=())]
fn test_inner_1(a: u32) {
}

#[etest(timeout=2_000, consumes="B")]
fn test_outer_1() {
    test_inner_1(42);
}



#[etest(timeout=2_000, skip=check_a(a), consumes="C", test_fn=())]
fn test_inner_2(a: u32) {
}

#[should_panic]
#[etest(timeout=2_000, uses="C")]
fn test_outer_2() {
    test_inner_2(23);
}


#[etest(timeout=1_000, skip=check_a(a), uses="D", test_fn=())]
fn test_inner_3(a: u32) {
}

#[should_panic]
#[etest(timeout=2_000, consumes="D")]
fn test_outer_3() {
    test_inner_3(23);
}
