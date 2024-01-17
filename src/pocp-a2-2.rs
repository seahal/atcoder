// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A02, Linear Search

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let fst = s
        .trim()
        .split_whitespace()
        .flat_map(|o| o.parse::<isize>())
        .last()
        .unwrap();

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let result = s
        .trim()
        .split_whitespace()
        .flat_map(|o| o.parse::<isize>())
        .any(|v| v == fst);

    println!("{}", if result { "Yes" } else { "No" });
}
