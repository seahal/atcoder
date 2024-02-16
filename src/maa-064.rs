// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 064 - All Zero

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let one: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .flatten()
        .collect();
    let n = one[0];
    let k = one[1];

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let ax: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .flatten()
        .collect();
    let sum = ax.iter().sum::<usize>();

    let ans = if k % 2 == sum % 2 && sum <= k {
        true
    } else {
        false
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
