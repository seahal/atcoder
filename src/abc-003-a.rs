// AtCoder Beginner Contest 003
// A - AtCoder社の給料

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap();

    println!("{}", (1 + n) * 10000 / 2);
}
