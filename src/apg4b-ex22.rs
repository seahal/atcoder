// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX22 - 2つ目の値でソート
use std::collections::BTreeMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut v: BTreeMap<isize, isize> = BTreeMap::new();

    for line in std::io::stdin().lines() {
        let mut l: Vec<isize> = line
            .unwrap()
            .split_whitespace()
            .flat_map(|v| v.parse::<isize>())
            .collect();

        v.insert(l[1], l[0]);
    }

    for (fst, snd) in v {
        println!("{snd} {fst}");
    }
}
