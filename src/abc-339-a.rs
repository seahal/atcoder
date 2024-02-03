// AtCoder Beginner Contest 339
// A - TLD

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut list = s.trim().split(" ");

    let h = list.next().unwrap().parse::<usize>().unwrap();
    let w = list.next().unwrap().parse::<usize>().unwrap();
    let n = list.next().unwrap().parse::<usize>().unwrap();

    println!("{}", h);
}
