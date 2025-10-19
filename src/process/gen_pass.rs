use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

// 字节切片
// b"..." ---> 生成带有 'static 生命周期的字节数组引用
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);

        // choose 是 rand 库提供的“从切片中随机挑选一个元素”的方法，属于一个扩展 trait 的方法，不是标准切片本身的固有方法
        // rand 0.9 版本中，choose 属于 IndexedRandom 这个 trait
        // 要想在 &[u8] 或 Vec 上调用 choose，必须把该 trait 引入作用域：use rand::seq::IndexedRandom;
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        // c 是一个引用（类型是 &u8），而 Vec::push 需要的是一个值（类型是 u8），所以要用 *c 把引用“解引用”成实际的字节值
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let pwd = String::from_utf8(password)?;

    println!("生成密码: {}", pwd);

    // 输出强度信息
    let estimate = zxcvbn(&pwd, &[]);

    eprintln!("强度评估: {}", estimate.score());

    Ok(())
}
