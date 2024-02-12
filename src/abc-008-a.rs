// AtCoder Beginner Contest(ABC), 008
// A - アルバム

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();

    println!("{}", x[1] - x[0] + 1);
}
