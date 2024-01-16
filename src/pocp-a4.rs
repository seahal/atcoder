// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A04, Binary Representation

fn main() {
    proconio::input! {
        n: u16,
    }

    // println!("{:010b}", &n);

    println!(
        "{}",
        (0..=9)
            .rev()
            .map(|i| &n / (1 << i) % 2)
            .fold("".to_string(), |fst, val| fst + &*val.to_string())
    );
}
