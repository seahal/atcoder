// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 059 - Power of Two

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n: usize = buffer.trim().parse().unwrap();

    let ans = match (n, n % 4) {
        (1, 0) => 2,
        (_, 0) => 6,
        (_, 1) => 2,
        (_, 2) => 4,
        (_, 3) => 8,
        _ => panic!(),
    };

    println!("{}", ans);
}
