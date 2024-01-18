// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A07 - Event Attendance - TLE
use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let d = s.trim().parse::<usize>().unwrap();

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let _n = s.trim().parse::<usize>().unwrap();

    let mut book = HashMap::<usize, usize>::new();

    for line in std::io::stdin().lines() {
        let l = line
            .expect("")
            .split_whitespace()
            .flat_map(|i| i.parse::<usize>())
            .collect::<Vec<_>>();
        for j in l[0]..=l[1] {
            if let Some(kttt) = book.get(&j) {
                book.insert(j, kttt + 1);
            } else {
                book.insert(j, 1);
            }
        }
    }

    for i in 1..=d {
        if let Some(j) = book.get(&i) {
            println!("{j}");
        } else {
            println!("0");
        }
    }
}
