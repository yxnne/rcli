## rcli 学习笔记

### clap：命令行工具

- 文档：`https://docs.rs/clap/latest/clap/index.html`
- 作用：把用户输入的 `argv` 变成强类型的 Rust 值，并自动生成 `--help`、错误提示等。

最小 Demo：

```rust
use clap::Parser;

#[derive(Parser)]
#[command(about = "A tiny demo")]
struct Cli {
    /// 要处理的文件
    input: std::path::PathBuf,
    /// 线程数
    #[arg(short, long, default_value_t = 4)]
    jobs: usize,
}

fn main() {
    let args = Cli::parse();
    println!("input={:?}, jobs={}", args.input, args.jobs);
}
```

`#[derive(Parser)]` 是 clap 的过程宏：
- 扫描字段类型、文档注释和 `#[arg(...)]` 属性；
- 生成 `clap::Parser` 的实现：解析 `args_os()`、默认值/约束校验、`--help` 与版本信息等。

### 子命令

```rust
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

    // subcommand 是 clap 派生宏定义的属性参数名，只在 #[command(...)] 中有效：
    // - 不带 subcommand：rcli csv ... / rcli genpass ...
    // - 带 subcommand：rcli base64 encode ... / rcli base64 decode ...（Base64SubCommand 内部还有更多子命令）
    #[command(subcommand)]
    Base64(Base64SubCommand),
}


#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}


#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file,  default_value = "-")]
    pub input: String,

    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

```

命令匹配关系：

```text
rcli csv input.csv -o out.json
  │   │    └──── 填进 CsvOpts
  │   └───────── 匹配到 Subcommand::Csv
  └───────────── 匹配到顶层 Opts

--- --- --- --- --- --- --- --- ---

rcli base64 encode input.txt
  │       │              │              └────── 填入 Base64EncodeOpts
  │       │              └──────────────── 匹配到 Base64SubCommand::Encode
  │       └────────────────────────────── 匹配到 Subcommand::Base64
  └────────────────────────────────────── 匹配到顶层 Opts
```

### `value_parser`

在解析命令行参数时，同时把原始字符串转换成合法值：
- 形式：`value_parser = some_fn`，把 `&str` 转成类型 `T`；
- 返回：`Result<T, E>`，其中 `E: std::fmt::Display`（`String`、`&str`、自定义错误都行）。

- 如果是普通文件路径：检查文件是否存在；不存在返回 `Err("xxx does not exist")`，clap 会显示：`error: Invalid value for '--input <INPUT>': xxx does not exist`。
- 如果用户给的是 `-`（`default_value = "-"`）：直接返回 `"-"`，表示“从标准输入读取”，业务侧判断 `input == "-"` 即可分支处理。

### 枚举与字符串转换的标准模式

另外，实现 value_parser 的时候可能会用到 parse 方法，需要实现一系列 trait。下面这一套代码是为 Base64Format 枚举实现字符串解析和格式化 的完整功能，让它能够在字符串和枚举值之间自由转换，属于 Rust 中处理枚举和字符串转换的标准模式：

- `format.parse::<Base64Format>()` 把字符串解析成 `Base64Format` 枚举：

```rust
// 目标：实现 Base64Format 的字符串解析/格式化，常用于 CLI 参数解析、配置读取等。

// - 字符串 → 枚举：依赖 FromStr 的实现
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse::<Base64Format>()
}

// 定义“字符串 → Base64Format”的规则
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

// 枚举值 → 字符串字面量
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

// 让枚举可用 Display 打印（println!/format!）
impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
```

### anyhow

[https://docs.rs/anyhow/latest/anyhow/](https://docs.rs/anyhow/latest/anyhow/)

`Result<T, anyhow::Error>`（俗称 `anyhow::Result<T>`）将错误类型统一为 `anyhow::Error`，避免在函数签名里暴露具体错误枚举；其它行为与标准 `Result<T, E>` 一致：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 等价关系：
// use anyhow::Result;
// Result<String> == Result<String, anyhow::Error>
```

| 维度     | 标准 `Result<T, E>`          | `anyhow::Result<T>`                 |
|----------|-------------------------------|-------------------------------------|
| 错误类型 | 具体类型 `E`                  | 统一为 `anyhow::Error`              |
| 函数签名 | 需要写明具体 `E`              | 统一写 `anyhow::Result<T>`          |
| 错误转换 | 需 `.map_err()`/自定义 `From` | 自动 `?` 转化，减少样板代码         |
| 信息展示 | 编译期可见具体错误类型        | 运行时可追根因，编译期不暴露类型    |
| 适用场景 | 库（给调用者精确错误）        | 应用（只想传播/打印错误）           |

### serde

数据序列化/反序列化库：结构体/枚举到 JSON/YAML/TOML 等的互转。常与 `serde_json`/`serde_yaml` 配合使用。

### rand

- 随机数/采样工具包。常见用法：`use rand::prelude::*; let mut rng = rand::rng();`
- 版本 0.9 变化：切片的 `choose` 方法由 `SliceRandom` 拆分到 `IndexedRandom`；需引入：`use rand::seq::IndexedRandom;`。

#### zxcvbn

密码强度评估库，可用于校验生成的密码是否易被猜测。

### base64

标准/URL 安全两种编码格式；本项目用 `Base64Format` 枚举管理，结合 `value_parser` 支持命令行选择。
