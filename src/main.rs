use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify, Base64SubCommand, Opts, Subcommand, TextSubCommand,
};

/// cargo add clap --features derive (只使用这个feature，clap：https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
fn main() -> anyhow::Result<()> {
    // cargo run csv --input assets/juventus.csv
    let options = Opts::parse();
    println!("{:?}", options);
    match options.cmd {
        // 调试eg: cargo run csv --input assets/juventus.csv --format yaml
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // 用 Rust 的 format! 宏构造一个字符串
                // - format!("output.{}", opts.format) 会返回一个新的 String，而不是打印到控制台
                // - 字符串模板 "output.{}" 里的 {} 是占位符，会被后面的参数 opts.format 替换
                // {} 使用的是 Display 格式化，如果类型没有实现 std::fmt::Display，这句代码会编译报错：缺少 Display 实现
                // format!("output.{}", opts.format) →
                // 调用 OutputFormat 的 Display →
                // Display 内部调用 Into::<&str>::into(*self) →
                // 使用 From for &'static str 完成到字符串的转换
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }

        // 调试eg: cargo run genpass --length 16
        Subcommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }

        // base64
        // cargo run -- base64 encode 自己输入 回车 后 ctrl + D 退出
        // cargo run -- base64 encode --format urlsafe -i Cargo.toml
        Subcommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                // process_base64_encode(&opts.input)?;
                println!("encode {}", opts.input);
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                // process_base64_decode(&opts.input)?;
                println!("decode {}", opts.input);
                process_decode(&opts.input, opts.format)?;
            }
        },

        // text 文本加密
        // eg: cargo run -- text sign -k fixtures/black3.txt
        Subcommand::Text(opts) => match opts {
            TextSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }

            TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, opts.format, &opts.signature)?;
            }
        },
    }

    Ok(())
}
