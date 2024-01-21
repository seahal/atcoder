// アルゴリズムと数学(ISBN: 978-4297125219)
// 008 - Brute Force 1

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let s: Vec<_> = s
        .trim()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect();

    let mut counter = 0;
    for i in 1..=s[0] {
        for j in 1..=s[0] {
            if i + j <= s[1] {
                counter += 1;
            }
        }
    }
    println!("{counter}");
}
