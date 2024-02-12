// AtCoder Beginner Contest(ABC) 064, https://atcoder.jp/contests/abc064/tasks/abc064_a
// A - RGB Cards

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("{}", to_yes_or_no(x % 4 == 0));
}

fn to_yes_or_no(b: bool) -> String {
    if b == true { "YES" } else { "NO" }.to_string()
}
