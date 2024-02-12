// AtCoder Beginner Contest(ABC), 016
// A - 12月6日

fn main() {
    let mut buffer = String::new();
    let mut x = std::io::stdin().read_line(&mut buffer);
    let md = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<isize>())
        .collect::<Vec<_>>();
    let m = md[0];
    let d = md[1];

    println!("{}", if m % d == 0 { "YES" } else { "NO" });
}
