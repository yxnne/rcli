use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file,  default_value = "-")]
    pub input: String,

    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file,  default_value = "-")]
    pub input: String,

    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

// 以下： 这一套代码是为 Base64Format 枚举实现
// 字符串解析和格式化 的完整功能，让它能够在字符串和枚举值之间自由转换
// 这是 Rust 中处理枚举和字符串转换的标准模式，常用于命令行参数解析、配置文件读取等场景。

// - 把字符串解析成 Base64Format 枚举
// - 内部调用 ： str::parse() 方法，依赖下面的 FromStr 实现
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse::<Base64Format>()
}

// 定义如何从字符串创建 Base64Format
impl std::str::FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid base64 format")),
        }
    }
}

// 把枚举值转换回字符串字面量
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

// 让枚举可以用 println!("{}", format) 或 format!("{}", format) 打印
impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
