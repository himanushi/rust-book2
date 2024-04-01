use clap::App;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {
        files: vec!["a".to_string()],
        lines: 1,
        bytes: None,
    })
}
