use custom_logger::env_logger_init;

fn main() {
    env_logger_init("warn");
    log::debug!("enter");

    println!("println output");
    log::info!("hello");

    log::debug!("exit");
}
