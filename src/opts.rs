use std::path::Path;

use clap::Parser;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input csv file", value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output json file", default_value = "output.json")]
    // 字面量转化为 String
    pub output: String,

    #[arg(long, help = "Has header", default_value_t = true)]
    pub header: bool,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
}

/// 验证文件是否存在
fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file does not exist")
    }
}
