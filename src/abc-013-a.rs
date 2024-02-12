// AtCoder Beginner Contest(ABC), 013
// A

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);

    let x = buffer.trim();
    println!("{:?}", x.as_bytes()[0] - 64);
}
