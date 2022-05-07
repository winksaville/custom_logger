# custom-logger

My typical custom logger which prints time in nanos as well as line numbers and thread id.
This must be used with an unstable build because `thread_id_value` uses `as_u64()`

## Building

```
cargo build
````

## Test

You must use `cargo test -- --nocapture` otherwise test captures the output can the tests will fail.
Another gotcha is that env_logger can only be initialized once so this is not a very thorough as only one configuration can be use.

TODO: Create crate tests/ and pass the default configuration on the command line to test other configurations.

```
wink@3900x 22-05-07T16:17:04.925Z:~/prgs/rust/myrepos/custom-logger (main)
$ cargo test -- --nocapture
   Compiling custom-logger v0.1.0 (/home/wink/prgs/rust/myrepos/custom-logger)
    Finished test [unoptimized + debuginfo] target(s) in 0.52s
     Running unittests src/lib.rs (target/debug/deps/custom_logger-c6ff9f644b370618)

running 2 tests
test test::test_env_logger_with_pipe ... 106 contents_so='println output
73 contents_se='[2022-05-07T16:19:03.847496514Z INFO  custom_logger::test   54  3] hello
'
'
73 contents_se='[2022-05-07T16:19:03.847543564Z INFO  custom_logger::test   76  2] hello
'
ok
test test::test_env_logger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests custom-logger

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@3900x 22-05-07T16:19:03.921Z:~/prgs/rust/myrepos/custom-logger (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
