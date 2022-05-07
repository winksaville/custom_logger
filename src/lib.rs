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
    use log;
    use stdio_override::{StdoutOverride, StderrOverride};
    use tempfile;
    use std::{fs, io::Result};
    use std::io::{prelude::*, stderr};
    use os_pipe::pipe;

    fn get_new_file_path() -> Result<tempfile::TempPath> {
        Ok(tempfile::NamedTempFile::new()?.into_temp_path())
    }

    #[test]
    fn test_env_logger_with_pipe() -> Result<()> {
        let (mut rx, tx) = pipe()?;

        env_logger_init("info");
        
        let guard_se = StderrOverride::from_io_ref(&tx)?;
        log::info!("hello");
        stderr().flush()?;
        drop(guard_se);
        drop(tx);

        let mut contents_se = String::new();
        rx.read_to_string(&mut contents_se)?;
        println!("{} contents_se='{}'", contents_se.len(), contents_se);
        assert!(contents_se.contains("hello"));

        Ok(())
    }

    #[test]
    fn test_env_logger() -> Result<()> {
        env_logger_init("info");
        
        let se_file_path = get_new_file_path()?;
        let so_file_path = get_new_file_path()?;
        let guard_so = StdoutOverride::from_file(&so_file_path)?;
        let guard_se = StderrOverride::from_file(&se_file_path)?;
        println!("println output");
        log::info!("hello");
        drop(guard_so);
        drop(guard_se);

        let contents_so = fs::read_to_string(&so_file_path)?;
        let contents_se = fs::read_to_string(&se_file_path)?;
        println!("{} contents_so='{}'", contents_so.len(), contents_so);
        println!("{} contents_se='{}'", contents_se.len(), contents_se);
        assert!(contents_so.contains("println output"));
        assert!(contents_se.contains("hello"));

        Ok(())
    }
}