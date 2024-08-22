//! Check that macros works without 'use etest::prelude::*'

#[etest::etest(test_fn=(), timeout=2_000, skip=true, uses="A", consumes=["B"])]
fn test_0() {
}
