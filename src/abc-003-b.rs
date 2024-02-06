// AtCoder Beginner Contest 003
// B - AtCoderトランプ

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let s = buffer.trim().to_string();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let t = buffer.trim().to_string();

    if s.len() != t.len() {
        return println!("{}", to_win_or_lose(false));
    }
    if s == t {
        return println!("{}", to_win_or_lose(true));
    }

    let ans = s
        .chars()
        .zip(t.chars())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| if a == '@' { (a, b) } else { (b, a) })
        .all(|(a, b)| a == '@' && "atcoder".chars().any(|c| c == b));
    println!("{}", to_win_or_lose(ans));
}

fn to_win_or_lose(b: bool) -> String {
    return if b {
        "You can win".to_string()
    } else {
        "You will lose".to_string()
    };
}

#[test]
fn test_to_win_or_lose() {
    assert_eq!(to_win_or_lose(true), "You can win");
    assert_eq!(to_win_or_lose(false), "You will lose");
}
