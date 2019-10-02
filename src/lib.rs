#![deny(rust_2018_idioms)]

pub use commands::run;
pub use errors::FastResult;
pub use rw::reader;

pub mod commands;
pub mod errors;
pub mod rw;
