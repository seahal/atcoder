// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX5 - A足すB問題
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut a = buffer.trim().split_whitespace().into_iter();
    if let (Some(x), Some(y)) = (a.next(), a.next()) {
        println!(
            "{}",
            x.parse::<isize>().unwrap() + y.parse::<isize>().unwrap()
        )
    }
}