// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 017 - Least Common Multiple of N Integers

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(char::is_whitespace)
        .flat_map(|i| i.parse::<usize>())
        .collect::<Vec<usize>>();

    println!("{}", lcm_vec(nums));
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

fn lcm(n: usize, m: usize) -> usize {
    n * m / gcd(n, m)
}

fn lcm_vec(nums: Vec<usize>) -> usize {
    nums.into_iter().reduce(|num, acc| lcm(num, acc)).unwrap()
}

#[test]
fn test_one() {
    debug_assert_eq!(lcm_vec(vec![24, 36, 54]), 72);
    debug_assert_eq!(lcm_vec(vec![12, 18, 14]), 252);
}
