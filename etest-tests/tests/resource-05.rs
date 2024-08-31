//! Tests 'None' resource type

use etest::prelude::*;

fn get_resource(v: u32) -> ResourceId {
    match v {
        0	=> None,
        1	=> Some("some-resource-1"),
        2	=> Some("some-resource-2"),
        _	=> unreachable!(),
    }.into()
}

#[etest(consumes=[get_resource(0), "inner-0-0"], test_fn=())]
fn test_inner_0_0() {
}

#[etest(timeout=2_000, consumes=[get_resource(0)])]
fn test_outer_0_0() {
    test_inner_0_0();
}

#[etest(consumes=[get_resource(1)], test_fn=())]
fn test_inner_0_1() {
}

#[should_panic]
#[etest(timeout=2_000, consumes=[get_resource(1)])]
fn test_outer_0_1() {
    test_inner_0_1();
}

//

#[etest(consumes=[get_resource(0)], test_fn=())]
fn test_inner_1_0() {
}

#[etest(timeout=2_000, uses=[get_resource(0)])]
fn test_outer_1_0() {
    test_inner_1_0();
}

#[etest(consumes=[get_resource(2)], test_fn=())]
fn test_inner_1_1() {
}

#[should_panic]
#[etest(timeout=2_000, uses=[get_resource(2)])]
fn test_outer_1_1() {
    test_inner_1_1();
}
