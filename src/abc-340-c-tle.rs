// AtCoder Beginner Contest 340
// C - Divide and Divide /
use std::collections::BinaryHeap;

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut money = 0;

    let mut v: BinaryHeap<usize> = BinaryHeap::new();
    v.push(n);

    loop {
        let x = v.pop().unwrap();
        v.push((x as f64 / 2.0).floor() as usize);
        v.push((x as f64 / 2.0).ceil() as usize);
        money += x;
        if v.iter().all(|&x| x <= 1) {
            break;
        }
    }
    println!("{}", money);
}
