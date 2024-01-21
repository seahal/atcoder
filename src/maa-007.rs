// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 007 - Number of Multiples 1

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let numbers: Vec<_> = s
        .trim()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect();

    let mut count = 0;

    for i in 1..=numbers[0] {
        if i % numbers[1] == 0 || i % numbers[2] == 0 {
            count += 1;
        }
    }
    println!("{:?}", count);
}
