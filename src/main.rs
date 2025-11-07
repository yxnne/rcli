use std::fs;

use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate_keye,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, Subcommand, TextSignFormat,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

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
            let pwd = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("生成密码: {}", pwd);
            // 输出强度信息
            let estimate = zxcvbn(&pwd, &[]);
            eprintln!("强度评估: {}", estimate.score());
        }

        // base64
        // cargo run -- base64 encode 自己输入 回车 后 ctrl + D 退出
        // cargo run -- base64 encode --format urlsafe -i Cargo.toml
        Subcommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                // process_base64_encode(&opts.input)?;
                // println!("encode {}", opts.input);
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                // process_base64_decode(&opts.input)?;
                // println!("decode {}", opts.input);
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },

        // text 文本加密
        // eg: cargo run -- text sign -k fixtures/black3.txt
        // eg: cargo run -- text verify -k fixtures/black3.txt --signature p5s9akpKJuDYUH96WfJKbRekgIsRRveVPy0aEHu-14o
        // eg: cargo run -- text generate -o fixtures
        // eg: cargo run -- text generate -o fixtures -f ed25519
        Subcommand::Text(opts) => match opts {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }

            TextSubCommand::Verify(opts) => {
                let verify =
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.signature)?;
                println!("{}", verify);
            }

            TextSubCommand::Generate(opts) => {
                // let key = process_text_generate_key(opts.format)?;
                // println!("{}", key);
                let key = process_text_generate_keye(opts.format)?;
                // println!("{}", key);

                match opts.format {
                    TextSignFormat::Black3 => {
                        let name = opts.output.join("black3.txt");
                        fs::write(name, &key[0])?;
                        // let key = Black3::new(key);
                        // println!("{}", key.key);
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                        // let key = Ed25519Signer::new(key);
                        // println!("{}", key.key);
                    }
                }
            }
        },
    }

    Ok(())
}
