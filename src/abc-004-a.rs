// Rust の勉強をするで

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    println!("{}", buffer.trim().parse::<isize>().unwrap() * 2);
}
