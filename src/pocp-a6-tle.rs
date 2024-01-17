// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A06 - How Many Guests?, TLE

use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let tx = s.split_whitespace().flat_map(|i| i.parse::<isize>());

    let mut date = HashMap::<isize, isize>::new();
    for (i, v) in tx.enumerate() {
        date.insert(i as isize + 1, v);
    }

    for j in std::io::stdin().lines() {
        let lr: Vec<_> = j
            .unwrap()
            .split_whitespace()
            .flat_map(|n| n.parse::<isize>())
            .collect();

        let y = (lr[0]..=lr[1]).map(|v| date[&v]).sum::<isize>();
        println!("{:}", y);
    }
}
