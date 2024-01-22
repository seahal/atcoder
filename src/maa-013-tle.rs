// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 012 - Primality Test

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let n = s.trim().parse::<isize>().unwrap();

    for i in 1..=(n / 2 + 1) {
        if n % i == 0 {
            println!("{i}");
        }
    }
    println!("{n}");
}
