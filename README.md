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
