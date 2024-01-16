// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// B04, 応用問題

fn main() {
    proconio::input! {
        n: String,
    }

    println!(
        "{}",
        n.chars()
            .rev()
            .enumerate()
            .map(|(i, v)| v.to_digit(10u32).unwrap() * 1u32 << i)
            .sum::<u32>()
    );
}
