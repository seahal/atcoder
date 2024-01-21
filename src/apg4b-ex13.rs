// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX13 - 平均との差

fn main() {
    let mut s = String::new();
    let _rr = std::io::stdin().read_line(&mut s);
    let length = s.trim().parse::<isize>().unwrap();

    s.clear();
    let _r = std::io::stdin().read_line(&mut s);
    let array = s.trim().split_whitespace().into_iter();
    let avg = array
        .clone()
        .fold(0, |fst, asc| asc.parse::<isize>().unwrap() + fst)
        / length;

    for point in array {
        let distance = (point.parse::<isize>().unwrap() - avg).abs();
        println!("{}", distance);
    }
}
