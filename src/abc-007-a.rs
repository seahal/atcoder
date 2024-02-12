// AtCoder Beginner Contest(ABC), 007
// A - 植木算

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<usize>().unwrap();
    println!("{}", x - 1);
}
