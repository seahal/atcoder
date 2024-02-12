// AtCoder Beginner Contest(ABC), 011
// A - 来月は何月？

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<isize>().unwrap();
    println!("{}", if x == 12 { 1 } else { x + 1 });
}
