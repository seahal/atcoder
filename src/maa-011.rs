// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 011 - Print Prime Numbers

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    println!(
        "{}",
        (1..=s.trim().parse::<isize>().unwrap())
            .filter(|n| is_prime(*n))
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn is_prime(n: isize) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }
    let x = (2..=((n as f64).sqrt().ceil() as isize)).find(|v| n % v == 0);
    x.is_none()
}
