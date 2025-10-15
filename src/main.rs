use clap::Parser;

use rcli::{process_csv, Opts, Subcommand};

/// cargo add clap --features derive (只使用这个feature，clap：https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
fn main() -> anyhow::Result<()> {
    // cargo run csv --input assets/juventus.csv
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opt) => {
            process_csv(&opt.input, &opt.output)?;
        }
    }

    Ok(())
}
