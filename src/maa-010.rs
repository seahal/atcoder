// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 010 - Factorial

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let n = s.trim().parse::<usize>().unwrap();

    println!("{}", factorial(n));
}

fn factorial(n: usize) -> usize {
    match n {
        1 | 0 => 1,
        _ => n * factorial(n - 1),
    }
}
