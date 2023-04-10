use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    // Todo: Don't create a BufWriter every time!
    let mut writer: Box<dyn Write> = if outfile.is_empty() {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(File::create(outfile)?)) // TODO: this doesn't work! a new file is created every time!
    };

    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false);
        }
        return Err(e);
    }

    Ok(true)
}
