use crate::FastResult;
use concat_reader::concat_path;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

/// Main input interface.
///
/// We wanted to allow users to read a directory or a file without having to choose a different
/// flag for each. This way users don't have to remember multiple flags and if they make a mistake
/// it's easy to adjust the target, for example by adding an extension.
///
/// This function uses the (concat_reader)[https://docs.rs/concat-reader/0.1.0/concat_reader/] crate
/// to read all the files in a directory into one buffer.
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
