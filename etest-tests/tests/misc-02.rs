//! Checks parsing of 'test_fn'

use etest::prelude::*;

#[test]
#[etest(test_fn=cfg(any()))]
fn test_0() {
    this_is_expected_to_be_ignored()
}

#[test]
#[etest(test_fn=cfg(all()))]
fn test_0() {
}
