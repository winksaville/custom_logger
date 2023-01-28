//#![feature(thread_id_value)]

use std::io::Write;

//// Use if/when thread_id_value is stablelized,
//// https://github.com/rust-lang/rust/issues/67939
//fn tid() -> u64 {
//    std::thread::current().id().as_u64().into()
//}

fn tid() -> u64 {

    unsafe {
        let id = std::thread::current().id();
        let addr: *const u64 = std::mem::transmute(&id);
        *addr

        //// Using transmute and Shadowning from [B] at
        ////   https://blog.knoldus.com/safe-way-to-access-private-fields-in-rust/
        //let id = std::thread::current().id();
        //struct MyThreadId(pub u64);
        //let my_id: MyThreadId = std::mem::transmute(id);
        //my_id.0
    }
}

/// Can only be called once
pub fn env_logger_init(default_level: &str) {
    let env = env_logger::Env::default();
    let mut xx = env_logger::Builder::from_env(env.default_filter_or(default_level));
    let yy = xx.format(|buf, record| {
        let time = std::time::SystemTime::now();
        writeln!(
            buf,
            "[{} {:5} {} {:>4} {:2}] {}",
            humantime::format_rfc3339_nanos(time),
            record.level(),
            if let Some(s) = record.module_path_static() {
                s
            } else {
                ""
            },
            if let Some(v) = record.line() { v } else { 0 },
            tid(),
            record.args()
        )
    });
    //.is_test(true)
    //.init();
    match yy.is_test(true).try_init() {
        Ok(_) => {}
        Err(_) => {}
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use gag::{Redirect, BufferRedirect};
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom},
    };

    #[allow(unused)]
    fn get_temp_filepath() -> String {
        return "/tmp/my_log.log".into();
    }

    #[allow(unused)]
    fn open_log_file() -> File {
        OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(get_temp_filepath())
            .unwrap()
    }

    #[allow(unused)]
    fn redirect_stderr_to_log_file() -> Redirect<File> {
        let log_file = open_log_file();
        Redirect::stderr(log_file).unwrap()
    }

    #[allow(unused)]
    fn stop_redirection_return_as_string(redirect: Redirect<File>) -> String {
        // Stop redirecting and return the log_file
        let mut log_file = redirect.into_inner();

        let mut buf_string = String::new();
        log_file.seek(SeekFrom::Start(0)).unwrap();
        log_file.read_to_string(&mut buf_string).unwrap();

        buf_string
    }

    #[allow(unused)]
    fn redirect_stderr_to_buffer() -> BufferRedirect {
        BufferRedirect::stderr().unwrap()
    }

    #[allow(unused)]
    fn stop_buffer_redirection_return_as_string(redirect: BufferRedirect) -> String {
        // Stop redirecting and return the string
        let mut log_file = redirect.into_inner();

        let mut buf_string = String::new();
        log_file.read_to_string(&mut buf_string).unwrap();

        buf_string
    }

    #[test]
    fn test_env_logger() {
        env_logger_init("info");

        //let redirect = redirect_stderr_to_log_file();
        let redirect = redirect_stderr_to_buffer();
        println!("println output");
        log::info!("hello");
        //let captured = stop_redirection_return_as_string(redirect);
        let captured = stop_buffer_redirection_return_as_string(redirect);
        println!("captured='{captured}'");
        assert!(captured.ends_with("] hello\n"));
    }
}
