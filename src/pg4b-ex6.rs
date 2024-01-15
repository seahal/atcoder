// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX6 - 電卓をつくろう
use std::io::stdin;

fn main() {
    let mut b = String::from("");
    stdin().read_line(&mut b).unwrap();
    let mut a = b.trim().split_whitespace();
    let left: isize = a.next().unwrap().parse().unwrap();
    let expr: char = a.next().unwrap().parse().unwrap();
    let right: isize = a.next().unwrap().parse().unwrap();

    match expr {
        '+' => println!("{}", left + right),
        '-' => println!("{}", left - right),
        '*' => println!("{}", left * right),
        '/' if right == 0 => println!("error"),
        '/' => println!("{}", left / right),
        '?' | '=' | '!' => println!("error"),
        _ => println!(""),
    }
}
