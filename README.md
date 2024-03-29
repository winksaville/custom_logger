# custom_logger

My typical custom logger which prints time in nanos as well as line numbers and thread id.

>Note: This has some `unsafe` code to be able to print thread id as a u64.
As far as I know, this will not have any UB but it is possible that the `tid`
could be garbage. Hopefully, `feature(thread_id_value)` will be stablized
someday. see [this tracking issue](https://github.com/rust-lang/rust/issues/67939).

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
custom_logger = { git = "https://github.com/winksaville/custom_logger", version = "0.2.0" }
log = "0.4.17"
```

Here is running src/main.rs:
```
wink@3900x 23-01-28T00:27:43.338Z:~/prgs/rust/myrepos/custom_logger (use-gag-to-capture)
$ cargo run
   Compiling custom_logger v0.2.0 (/home/wink/prgs/rust/myrepos/custom_logger)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/custom_logger`
println output
[2023-01-28T00:27:46.951127343Z INFO  custom_logger    7  1] hello
```

## Building

```
cargo build
```

## Test

You must use `cargo test -- --nocapture` otherwise test captures the output and the tests will fail.
Another gotcha is that env_logger can only be initialized once so this is not thoroughly tests as
only one configuration can be use.

>Note: I'm guessing if you use the `process::Command` technique that env_logger none of the output
will be captured by the test harness and instead will be captured by the code. This will make the
test better and more reliable. This will also allow other configurations to be used and mentioned
in the todo below.

TODO: Create crate tests/ and pass the default configuration on the command line to test other configurations.

```
wink@3900x 23-01-28T00:29:21.787Z:~/prgs/rust/myrepos/custom_logger (use-gag-to-capture)
$ cargo test -- --nocapture
   Compiling custom_logger v0.2.0 (/home/wink/prgs/rust/myrepos/custom_logger)
    Finished test [unoptimized + debuginfo] target(s) in 0.44s
     Running unittests src/lib.rs (target/debug/deps/custom_logger-cc98325a4823cf2e)

running 1 test
println output
captured=[2023-01-28T00:29:32.909674995Z INFO  custom_logger::test  101  2] hello

test test::test_env_logger ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/custom_logger-8f6ee98ac00e27fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests custom_logger

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
