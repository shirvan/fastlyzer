use crate::FastResult;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};
use concat_reader::concat_path;

pub fn reader(file: &str) -> FastResult<Box<dyn BufRead>> {
    let mut res = vec![];
    let path = Path::new(file);
    if path.is_dir() {
        for file in path.read_dir() {
            let mut paths = file
                .into_iter()
                .map(|f| f.unwrap().path())
                .collect();

            let mut f = concat_path(paths);
            let mut con = BufReader::new(f);
            Ok(Box::new(con))
        }
    } else {
        let file = OpenOptions::new().read(true).open(&path)?;
        let buff = BufReader::new(file);
        Ok(Box::new(buff))
    }
}
