use std::{fmt, path::Path, str::FromStr};

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
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input csv file", value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output json file")]
    // 字面量转化为 String
    pub output: Option<String>,

    #[arg(long, help = "Output format", value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

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

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // .parse() 会自动使用为 OutputFormat 实现的 FromStr（见下面的 impl FromStr for OutputFormat）
    format.parse()
}

// 如何从字符串解析成 OutputFormat 枚举
// 接受字符串 s，如果是 "json" 返回 OutputFormat::Json；
// "yaml" 返回 OutputFormat::Yaml；
// 其他返回错误 anyhow!("Invalid format")
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// 把枚举值转成对应的字符串字面量
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", ...) 是格式化输出宏，作用是把格式化后的内容写到 f（fmt::Formatter）
        // Into::<&str>::into(*self) 做的是类型转换:Into trait 把 OutputFormat 转成 &str
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
