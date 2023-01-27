use custom_logger::env_logger_init;

fn main() {
    env_logger_init("info");

    println!("println output");
    log::info!("hello");
}
