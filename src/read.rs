use crate::CHUNK_SUZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read(infile: &str) -> Result<Vec<u8>> {
    // Todo: Don't create a BufReader every time!
    let mut reader: Box<dyn Read> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?)) // TODO: This will only read the first 16k every time!
    };

    let mut buffer = [0; CHUNK_SUZE];
    let num_read = reader.read(&mut buffer)?;

    Ok(Vec::from(&buffer[..num_read]))
}
