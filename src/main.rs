use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::get();
    let mut total_bytes = 0;
    let infile = args.infile.unwrap_or_default();
    let outfile = args.outfile.unwrap_or_default();
    let silent = args.silent;

    loop {
        let buffer = match read::read(&infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        stats::stats(silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&outfile, &buffer)? {
            break;
        }
    }
    stats::stats(silent, 0, &mut total_bytes, true);

    Ok(())
}
