// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A04, Binary Representation - rewrite

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let n: isize = s.trim().parse::<isize>().unwrap();

    // println!("{:010b}", &n);

    println!(
        "{}",
        (0..=9)
            .rev()
            .map(|i| &n / (1 << i) % 2)
            .fold("".to_string(), |fst, val| fst + &*val.to_string())
    );
}
