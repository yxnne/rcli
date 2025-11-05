use anyhow::Result;

pub fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    // `if` and `else` have incompatible types expected `Stdin`, found `File`
    // Box<dyn std::io::Read> 封装到box里面是一种常见的模式，用于处理不同类型
    let reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}
