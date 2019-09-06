use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path
};
use crate::Result;

pub fn read(file: &str) -> Result<Box<dyn BufRead>> {
    let path = Path::new(file);
    let file = OpenOptions::new().read(true).open(&path)?;
    Ok(Box::new(BufReader::new(file)))
}

//pub fn write(strings: &str) {
//    let mut tw = TabWriter::new(io::stdout());
//    write!(&mut tw, "{}", strings).unwrap();
//    tw.flush().unwrap();
//}
