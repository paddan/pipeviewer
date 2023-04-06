use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

use clap::Parser;

const CHUNK_SUZE: usize = 16 * 1024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Read from a file instead of stdin
    infile: Option<String>,

    /// Write to a file instead of stdout
    #[arg(long, short)]
    outfile: Option<String>,

    /// Silent output to stderr
    #[arg(short, long)]
    silent: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let infile = args.infile.unwrap_or_default();
    let outfile = args.outfile.unwrap_or_default();
    let silent = if args.silent {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    dbg!(infile, outfile, silent);

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SUZE];
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{total_bytes}");
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        eprint!("\r{total_bytes}");
    }
    Ok(())
}
