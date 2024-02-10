// AtCoder Beginner Contest 005
// B - おいしいたこ焼きの作り方
use std::collections::BTreeSet;

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<isize>().unwrap();

    let mut bts: BTreeSet<isize> = BTreeSet::new();
    for _ in 1..=n {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        bts.insert(buffer.trim().parse::<isize>().unwrap());
    }
    println!("{}", bts.first().unwrap());
}
