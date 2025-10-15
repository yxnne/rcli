use std::{fs, path::Path};

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// cargo add clap --features derive (只使用这个feature，clap：https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opt) => {
            let mut reader = Reader::from_path(opt.input)?;
            // ? 相当于做了match
            // match reader {
            //     Ok(v) => ...
            //     Err(e) => return Err(e.into()),
            // }

            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                // println!("{:?}", record);
                ret.push(record);
            }

            let json = serde_json::to_string_pretty(&ret)?;

            fs::write(opt.output, json)?;
            // let records = reader
            //     .deserialize()
            //     .map(|result| result.unwrap())
            //     .collect::<Vec<Player>>();

            // println!("{:?}", records);
        }
    }

    Ok(())
}
#[derive(Debug, Parser)]
#[command(name = "rcli", about = "A simple command line tool", version, author)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Convert csv to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help = "Input csv file", value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, help = "Output json file", default_value = "output.json")]
    // 字面量转化为 String
    output: String,

    #[arg(long, help = "Has header", default_value_t = true)]
    header: bool,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    delimiter: char,
}

/// 验证文件是否存在
fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file does not exist")
    }
}
