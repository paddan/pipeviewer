//! The stats module contains the stats loop
//!
//! # some header
//! More discussion!

use crossbeam::channel::Receiver;
use crossterm::style::Stylize;
use crossterm::{
    cursor, execute,
    style::{self, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Stderr, Write};
use timer::Timer;

pub mod timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        total_bytes += num_bytes;
        let total_rate_per_second = total_bytes as f64 / timer.total.as_secs_f64();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                timer.start.elapsed().as_secs().as_time(),
                rate_per_second,
                total_rate_per_second,
            );
        }

        if num_bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64, total_rate: f64) {
    let bytes = style::style(format!("{:.0} Mb ", bytes / 1024 / 1024)).red();
    let elapsed = style::style(elapsed).green();
    let rate = style::style(format!(" [{:.0} Mb/s]", rate / 1024_f64 / 1024_f64)).blue();
    let total_rate = style::style(format!(" ({:.0} Mb/s)", total_rate / 1024_f64 / 1024_f64)).dark_blue();
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate),
        PrintStyledContent(total_rate)
    );
    let _ = stderr.flush();
}

/// The TimeOutput trait adds a `.as_time()` method to `u64`
///
/// # Example
/// ```rust
/// use pipeviewer::stats::TimeOutput;
/// use pipeviewer::timer::TimeOutput;
/// assert_eq!(65_u64.as_time(), String::from("0:01:05"))
/// ```
pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3603_u64, "1:00:03"),
        ];

        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
