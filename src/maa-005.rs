// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 005 - Modulo 100

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let s: isize = s
        .trim()
        .split_whitespace()
        .flat_map(|v| v.parse::<isize>())
        .sum();
    println!("{}", s % 100);
}
