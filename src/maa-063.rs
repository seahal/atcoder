// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 063 - Move on Squares 2

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n: usize = buffer.trim().parse().unwrap();

    let ans = match n % 2 {
        0 => true,
        _ => false,
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
