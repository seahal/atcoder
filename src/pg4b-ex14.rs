// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX14 - 三人兄弟の身長差

use itertools::Itertools;

fn main() {
    let mut s = String::new();
    let r = std::io::stdin().read_line(&mut s);
    let mut array = s.split_whitespace().into_iter().map(|v| v.parse::<isize>());

    let def = array.nth(0).unwrap().unwrap();
    let (mut min, mut max) = (def, def);

    for a in array {
        if max < a.clone().unwrap() {
            max = a.clone().unwrap();
        }
        if min > a.clone().unwrap() {
            min = a.clone().unwrap();
        }
    }
    println!("{}", max - min);
}
