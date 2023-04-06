use crate::CHUNK_SUZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read(infile: &Option<String>) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if let Some(infile) = infile {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SUZE];
    let num_read = reader.read(&mut buffer)?;

    Ok(Vec::from(&buffer[..num_read]))
}
