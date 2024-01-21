// AtCoder Beginner Contest 086
// B - 1 21

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let i = s
        .trim()
        .split(' ')
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut x = (1..=1000).filter(|n| n * n == i);

    if let Some(_) = x.next() {
        println!("Yes");
    } else {
        println!("No");
    }
}
