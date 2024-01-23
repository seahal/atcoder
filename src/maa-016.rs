// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 016 - Greatest Common Divisor of N Integers

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let nums = s
        .split(char::is_whitespace)
        .flat_map(|i| i.parse::<usize>());

    println!("{}", nums.fold(0, |num, acc| gcd(num, acc)));
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    if n > m {
        let t = m;
        m = n;
        n = t;
    }

    if n == 0 {
        m
    } else {
        return gcd(m % n, n);
    }
}
