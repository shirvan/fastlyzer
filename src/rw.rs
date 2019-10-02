use crate::FastResult;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};
use concat_reader::concat_path;

pub fn reader(file: &str) -> FastResult<Box<dyn BufRead>> {
    let path = Path::new(file);
    if path.is_dir() {
        let paths = path.read_dir()
            .into_iter()
            .map(|file| {
                file.into_iter()
                    .map(|f| f.unwrap().path())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        let mut f = concat_path(paths);
        let mut con = BufReader::new(f);
        Ok(Box::new(con))
    } else {
        let file = OpenOptions::new().read(true).open(&path)?;
        let buff = BufReader::new(file);
        Ok(Box::new(buff))
    }
}
