use clap::Parser;

/// cargo add clap --features derive (只使用这个feature，clap：https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
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
    #[arg(short, long, help = "Input csv file")]
    input: String,

    #[arg(short, long, help = "Output json file", default_value = "output.json")]
    // 字面量转化为 String
    output: String,

    #[arg(long, help = "Has header", default_value_t = true)]
    header: bool,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    delimiter: char,
}
