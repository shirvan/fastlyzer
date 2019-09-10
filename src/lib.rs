use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufRead, Write};
use tabwriter::TabWriter;

use entry::Entry;
pub use errors::FastResult;
pub use rw::read;
pub use commands::get_domains;

pub mod entry;
pub mod errors;
pub mod rw;
pub mod commands;
