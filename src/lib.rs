#![deny(rust_2018_idioms)]
//! # Fastlyzer
//!
//! Fastlyzer is a JSON log analyzer. It aims to be the fastest and the most feature complete
//! CLI JSON log analyzer.
pub use commands::run;
pub use errors::FastResult;
pub use rw::reader;

pub mod commands;
pub mod errors;
pub mod rw;
