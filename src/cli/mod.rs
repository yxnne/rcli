mod base64;
mod csv;
mod genpass;

use std::path::Path;

use clap::Parser;

// use crate::cli::csv::CsvOpts;
// use self::csv::CsvOpts;
// use self::genpass::GenPassOpts;
// - self ：当前模块
// - super ：父模块
// - crate ：当前 crate 的根模块
use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

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

    //- 不带 subcommand 的：
    // 用户输入 rcli csv ... 或 rcli genpass ...
    // - 带 subcommand 的：
    // 用户输入 rcli base64 encode ... 或 rcli base64 decode ... （Base64SubCommand 内部还有更多子命令）
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

/// 验证文件是否存在
fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    // 判断input 是 - 或者文件存在
    if file_name == "-" || Path::new(file_name).exists() {
        // Ok(file_name.to_string())
        // - 在 Rust 中，函数的最后一行如果没有分号，就是该函数的返回值
        // - 如果加了分号，那行就变成了一个语句（statement），返回 () （unit type）
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

// 有在运行测试时（ cargo test ）才会编译和包含这部分代码
// 好处:
// - 减小二进制文件大小 ：测试代码不会被编译到发布版本中
// - 避免依赖冲突 ：测试专用的依赖不会影响生产代码
// - 组织清晰 ：明确区分测试代码和业务代码

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
