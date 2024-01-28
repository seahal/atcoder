// AtCoder Beginner Contest 338
// B - Frequency
use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut hash = HashMap::new();
    let mut array = s.trim().chars().into_iter();
    for c in array {
        hash.entry(c).and_modify(|curr| *curr += 1).or_insert(1);
    }

    let mut nums: Vec<(&char, &i32)> = hash.iter().collect();
    let x = nums.sort_by(|lft: &(&char, &i32), rgt: &(&char, &i32)| {
        rgt.1.cmp(&lft.1).then(lft.0.cmp(&rgt.0))
    });

    println!("{}", nums[0].0);
}
