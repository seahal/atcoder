// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A05 - Three Cards

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut colors = s.split_whitespace().flat_map(|o| o.parse::<isize>());
    let n = colors.next().unwrap();
    let k = colors.next().unwrap();
    let mut count: usize = 0;

    for i in 1..=n {
        for j in 1..=n {
            if k > i + j && k - (i + j) <= n {
                count += 1;
            }
        }
    }

    println!("{count}");
}
