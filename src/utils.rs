use super::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Windows API error")]
    Windows(#[from] windows::core::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub fn init_logging() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
}