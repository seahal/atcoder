// AtCoder Beginner Contest 005
// A - おいしいたこ焼きの作り方

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let nums = buffer
        .split_whitespace()
        .flat_map(|x| x.parse::<f64>())
        .collect::<Vec<_>>();
    println!("{}", (nums[1] / nums[0]).floor());
}
