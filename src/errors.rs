use failure::Fail;
use std::io;

/// Error variants
#[derive(Fail, Debug)]
pub enum FastlyzerErr {
    /// IO Errors
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serde errors
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
}

impl From<io::Error> for FastlyzerErr {
    fn from(err: io::Error) -> FastlyzerErr {
        FastlyzerErr::Io(err)
    }
}

impl From<serde_json::Error> for FastlyzerErr {
    fn from(err: serde_json::Error) -> FastlyzerErr {
        FastlyzerErr::Serde(err)
    }
}

/// Fastlyzer Result type
pub type FastResult<T> = std::result::Result<T, FastlyzerErr>;
