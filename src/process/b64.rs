use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Format;
use anyhow::Result;

fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    // `if` and `else` have incompatible types expected `Stdin`, found `File`
    // Box<dyn std::io::Read> 封装到box里面是一种常见的模式，用于处理不同类型
    let reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // `if` and `else` have incompatible types expected `Stdin`, found `File`
    // Box<dyn std::io::Read> 封装到box里面是一种常见的模式，用于处理不同类型
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    // let encoded = URL_SAFE.encode(buf);

    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
        Base64Format::Standard => STANDARD.encode(buf),
    };
    println!("{}", encoded);

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => STANDARD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_encode() {
    //     let input = "hello world";
    //     let encoded = process_encode(input, Base64Format::Standard).unwrap();
    //     // assert_eq!(encoded, ());
    //     // assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    // }

    // #[test]
    // fn test_decode() {
    //     let input = "aGVsbG8gd29ybGQ=";
    //     let decoded = process_decode(input, Base64Format::Standard).unwrap();
    //     // assert_eq!(decoded, "hello world");
    // }
}
