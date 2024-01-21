// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX23 - 最頻値

use std::collections::BTreeMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    if s.trim() == "0" {
        println!("0 0");
        std::process::exit(0);
    }

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let numbers = s.trim().split(' ').flat_map(|v| v.parse::<isize>());

    let mut dictorinay: BTreeMap<isize, isize> = BTreeMap::new();
    for i in numbers {
        dictorinay.insert(i, dictorinay.get(&i).unwrap_or(&0) + 1);
    }

    if s.len() == 0 {
        println!("0 0");
        std::process::exit(0);
    }

    let (x, y) = dictorinay.iter().last().unwrap();
    println!("{} {}", x, y);
}
