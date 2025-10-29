mod csv;
mod genpass;

use clap::Parser;

// use crate::cli::csv::CsvOpts;
// use self::csv::CsvOpts;
// use self::genpass::GenPassOpts;
use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", about = "A simple command line tool", version, author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert csv to json")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    // #[command(name = "base64", about = "Base64 encode/decode")]
    // Base64(Base64SubCommand),
}
