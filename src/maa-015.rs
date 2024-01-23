// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 015 - Greatest Common Divisor

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut nums = s
        .split(char::is_whitespace)
        .flat_map(|i| i.parse::<usize>());

    let fst = nums.next().unwrap();
    let snd = nums.next().unwrap();
    println!("{}", gcd(fst, snd));
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

#[test]
fn test_one() {
    debug_assert_eq!(gcd(33, 88), 11);
    debug_assert_eq!(gcd(123, 777), 3);
}
