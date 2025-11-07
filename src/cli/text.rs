use std::{
    fmt::{self, Display},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};
use clap::Parser;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "sign a message with a secret key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "verify a singed message")]
    Verify(TextVerifyOpts),

    #[command(about = "generate a key pair")]
    Generate(TextGenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file,  default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file )]
    pub key: String,

    #[arg(long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file,  default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file )]
    pub key: String,

    #[arg(short, long)]
    pub signature: String,

    #[arg(long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextGenerateKeyOpts {
    #[arg(short, long, default_value = "black3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Black3,
    Ed25519,
}

fn parse_text_sign_format(f: &str) -> Result<TextSignFormat> {
    f.parse()
}

impl FromStr for TextSignFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black3" => Ok(TextSignFormat::Black3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow!("Invalid text sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Black3 => "black3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
