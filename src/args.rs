use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Read from a file instead of stdin
    pub infile: Option<String>,

    /// Write to a file instead of stdout
    #[arg(long, short)]
    pub outfile: Option<String>,

    /// Silent output to stderr
    #[arg(short, long)]
    pub silent: bool,
}
//
// impl Args {
//     pub fn parse() -> Self {
//         Self::parse()
//     }
// }
