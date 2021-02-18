use std::io;

use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum Error {
    Io(io::Error),
    Http(http::Error),
    HttpUri(http::uri::InvalidUri),
    Hyper(hyper::Error),
    #[error(ignore)]
    HyperStatus(hyper::StatusCode),
    Json(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
