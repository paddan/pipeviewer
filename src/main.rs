use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use crossbeam::channel::{bounded, unbounded};
use std::thread;

fn main() -> Result<()> {
    let args = Args::get();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile.unwrap_or_default(), stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle =
        thread::spawn(move || write::write_loop(&outfile.unwrap_or_default(), write_rx));

    read_handle.join().unwrap()?;
    stats_handle.join().unwrap()?;
    write_handle.join().unwrap()?;

    Ok(())
}
