#![feature(thread_id_value)]

use std::io::Write;

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
            std::thread::current().id().as_u64(),
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
    use gag::Redirect;
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom},
    };

    fn get_temp_filepath() -> String {
        return "/tmp/my_log.log".into();
    }

    fn open_log_file() -> File {
        OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(get_temp_filepath())
            .unwrap()
    }

    fn redirect_stderr_to_log_file() -> Redirect<File> {
        let log_file = open_log_file();
        Redirect::stderr(log_file).unwrap()
    }

    fn stop_redirection_return_as_string(redirect: Redirect<File>) -> String {
        // Stop redirecting and return the log_file
        let mut log_file = redirect.into_inner();

        let mut buf_string = String::new();
        log_file.seek(SeekFrom::Start(0)).unwrap();
        log_file.read_to_string(&mut buf_string).unwrap();

        buf_string
    }

    #[test]
    fn test_env_logger() {
        env_logger_init("info");

        let redirect = redirect_stderr_to_log_file();
        println!("println output");
        log::info!("hello");
        let captured = stop_redirection_return_as_string(redirect);
        println!("captured={captured}");
        assert!(captured.ends_with("] hello\n"));
    }
}
