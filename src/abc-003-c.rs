// AtCoder Beginner Contest 004
// C - 入れ替え,

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap();
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    for i in 0..(n % 30) {
        let j = i % 5 + 1;
        nums.swap(j - 1, j);
    }

    for i in &nums {
        print!("{i}");
    }
    print!("\n");
}
