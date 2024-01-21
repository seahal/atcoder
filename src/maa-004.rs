// アルゴリズムと数学(ISBN: 978-4297125219)
// 004 - Product of 3 Integers

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let s: isize = s
        .trim()
        .split_whitespace()
        .flat_map(|v| v.parse::<isize>())
        .product();
    println!("{}", s);
}
