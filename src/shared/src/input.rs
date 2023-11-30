use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

pub struct AocBufReader {
    iter: Lines<BufReader<File>>,
}

impl AocBufReader {
    fn from_file(file_handle: File) -> AocBufReader {
        AocBufReader {
            iter: BufReader::new(file_handle).lines(),
        }
    }

    pub fn from_string(file_path: &str) -> AocBufReader {
        AocBufReader::from_file(open_file(file_path))
    }
}

impl Iterator for AocBufReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(result) => match result {
                Ok(line) => Some(line),
                Err(error) => panic!("{}", error),
            },
            None => None,
        }
    }
}
