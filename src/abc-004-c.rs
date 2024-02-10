// AtCoder Beginner Contest 004
// C - 入れ替え,

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap() % 30;
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    for i in 0..n {
        let j = i % 5 + 1;
        let fst = nums[j - 1];
        let snd = nums[j];
        nums[j - 1] = snd;
        nums[j] = fst;
    }

    for i in &nums {
        print!("{i}");
    }
    print!("\n");
}
