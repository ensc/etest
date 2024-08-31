//! Tests 'no_default_uses' parameter

use etest::prelude::*;

#[etest(timeout=3_000, notparallel, test_fn=())]
fn test_inner_0() {
}

#[etest(timeout=4_000, no_default_uses)]
fn test_outer_0() {
    test_inner_0()
}


#[etest(timeout=1_000, notparallel, test_fn=())]
fn test_inner_1() {
}

#[should_panic]
#[etest(timeout=2_000)]
fn test_outer_1() {
    test_inner_1()
}
