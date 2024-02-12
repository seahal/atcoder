// AtCoder Beginner Contest(ABC), 006
// A - 世界のFizzBuzz, https://atcoder.jp/contests/abc006/tasks/abc006_1

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<usize>().unwrap();
    println!("{}", is_dividable_by_3(x));
}

fn is_dividable_by_3(n: usize) -> String {
    if n % 3 == 0 { "yes" } else { "no" }.to_uppercase()
}
