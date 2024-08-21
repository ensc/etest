[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Coverage Status][coveralls-badge]][coveralls-url]
[![Build Status][ci-badge]][ci-url]

[crates-badge]:    https://img.shields.io/crates/v/etest.svg
[crates-url]:      https://crates.io/crates/etest
[docs-badge]:      https://img.shields.io/docsrs/etest/latest
[docs-url]:        https://docs.rs/etest/latest/etest/index.html
[coveralls-badge]: https://coveralls.io/repos/github/ensc/etest/badge.svg?branch=master
[coveralls-url]:   https://coveralls.io/github/ensc/etest?branch=master
[ci-badge]:        https://github.com/ensc/etest/actions/workflows/rust.yml/badge.svg
[ci-url]:          https://github.com/ensc/etest/actions/workflows/rust.yml

# About

`etest` allows to control behaviour of `#[test]` like functions:

- it allows to specify the maximum runtime of a test (`timeout` parameter)

- it allows to skip a test based on the result of another function
  (`skip_*` parameter class)

- it allows to limit parallel execution of tests by making it dependend
  on resources which are simply used ("shared" usage) or consumed for
  the duration of test ("exclusive" usage)

# Example

```rust
#[etest(skip=is_today_monday(),
        timeout=2_000,
        consumes=["video", "audio"],
        uses=["network"])]
fn test_00() {
    do_something();
}
```

This test will:

- be skipped at monday

- will be aborted after 2 seconds

- will use "video" and "audio" resources exclusively; e.g. there will
  be no other test running which uses one or both of these resources

- will share the "network" resource; e.g. other tests which share this
  resource are allowed at the same time, but no tests which use it
  exclusively.

# License

The code of the toplevel `etest` crate (content of `src` folder) is
licensed under LGPL-3.0-or-later with an exception which removes
restrictions regarding static linking.

Implementation details (crates in the `etest-derive` + `etest-impl`
folders) are licensed under GPL-3.0-or-later with the explicit
permission to use and distribute the generated code (the expanded
macro) under terms of your choice.

Independent tests (`etest-tests`) are licensed under GPL-3.0-or-later.
