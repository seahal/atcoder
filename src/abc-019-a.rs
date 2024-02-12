// AtCoder Beginner Contest(ABC), 019
// A - 高橋くんと年齢

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut ages: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect();

    ages.sort();
    println!("{}", ages[1]);
}
