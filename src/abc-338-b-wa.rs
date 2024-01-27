// AtCoder Beginner Contest 338
// B - Frequency

use std::collections::BTreeMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut hash = BTreeMap::new();
    let mut array = s.trim().chars().into_iter();
    for c in array {
        hash.entry(c).and_modify(|curr| *curr += 1).or_insert(1);
    }

    let mut hash_two = BTreeMap::new();
    for h in &hash {
        hash_two.insert(h.1, h.0);
    }

    println!("{}", hash_two.pop_last().unwrap().1);
}
