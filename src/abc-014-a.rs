// AtCoder Beginner Contest(ABC), 014
// A - けんしょう先生のお菓子配り
use std::io::{self, BufRead};

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    let _ = std::io::stdin().lock().read_line(&mut buffer);
    let y = buffer.trim().parse::<usize>().unwrap();

    let r#mod = x % y; // えっ、Rust って # も変数名に含められるの？

    if r#mod == 0 {
        println!("{}", 0);
    } else {
        println!("{}", y - r#mod);
    }
}
