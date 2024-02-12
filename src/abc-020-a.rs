// AtCoder Beginner Contest(ABC), 020
// A - クイズ

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let ans: isize = buffer.trim().parse::<isize>().unwrap();
    println!("{}", if ans == 1 { "ABC" } else { "chokudai" });
}
