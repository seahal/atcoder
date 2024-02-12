// AtCoder Beginner Contest 003
// A

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    println!("{}", buffer.trim().parse::<isize>().unwrap() * 2);
}
