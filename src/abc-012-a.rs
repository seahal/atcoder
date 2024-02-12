// AtCoder Beginner Contest(ABC), 012
// A - スワップ

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);

    let mut x = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();
    let mut a = x[0];
    let mut b = x[1];

    let temp = b;
    b = a;
    a = temp;

    println!("{} {}", a, b);
}
