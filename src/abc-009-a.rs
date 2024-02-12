// AtCoder Beginner Contest(ABC), 009
// A - スワップ

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<f64>().unwrap();
    println!("{}", (x / 2.0).ceil())
}
