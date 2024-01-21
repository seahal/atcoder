// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 001 - Print 5+N

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    println!("{:?}", s.trim().parse::<isize>().unwrap_or_default() + 5);
}
