# custom-logger

My typical custom logger which prints time in nanos as well as line numbers and thread id.
This must be used with an unstable build because `thread_id_value` uses `as_u64()`

## Usage

Example:
```
use custom_logger::env_logger_init;

fn main() {
    env_logger_init("info");

    println!("println output");
    log::info!("hello");
}
```

Dependencies:
```
[dependencies]
custom-logger = { git = "https://github.com/winksaville/custom-logger", version = "0.2.0" }
log = "0.4.17"
```


## Building

```
cargo build
```

## Test

You must use `cargo test -- --nocapture` otherwise test captures the output can the tests will fail.
Another gotcha is that env_logger can only be initialized once so this is not a very thorough as only one configuration can be use.

TODO: Create crate tests/ and pass the default configuration on the command line to test other configurations.

```
wink@3900x 23-01-27T23:45:20.791Z:~/prgs/rust/myrepos/custom-logger (use-gag-to-capture)
$ cargo test -- --nocapture
   Compiling custom-logger v0.1.0 (/home/wink/prgs/rust/myrepos/custom-logger)
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (target/debug/deps/custom_logger-639fef1e42777d75)

running 1 test
println output
captured=[2023-01-27T23:45:27.767671827Z INFO  custom_logger::test   76  2] hello

test test::test_env_logger ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/custom_logger-a05169731f72476b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests custom-logger

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
