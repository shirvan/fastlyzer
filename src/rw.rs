use crate::FastResult;
use concat_reader::concat_path;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn reader(file: &str) -> FastResult<Box<dyn BufRead>> {
    let path = Path::new(file);
    if path.is_dir() {
        let paths = path
            .read_dir()
            .into_iter()
            .map(|file| file.map(|f| f.unwrap().path()).collect::<Vec<_>>())
            .flatten()
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();
        let file = concat_path(paths);
        let buff = BufReader::new(file);
        Ok(Box::new(buff))
    } else {
        let file = OpenOptions::new().read(true).open(&path)?;
        let buff = BufReader::new(file);
        Ok(Box::new(buff))
    }
}
