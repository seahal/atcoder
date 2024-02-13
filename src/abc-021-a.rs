// AtCoder Beginner Contest(ABC), 020
// A 足し算

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let num: usize = buffer.trim().parse().unwrap();
    println!("{num}");
    for j in 1..=num {
        println!("1");
    }
}
