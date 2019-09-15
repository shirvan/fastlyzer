#![deny(rust_2018_idioms)]

pub use commands::get_domains;
pub use errors::FastResult;
pub use rw::read;

pub mod commands;
pub mod errors;
pub mod rw;
