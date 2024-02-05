// AtCoder Beginner Contest (ABC), 001
// A - 積雪深差

fn main() {
    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);
    let fst = buff.trim().parse::<isize>().unwrap();
    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);
    let snd = buff.trim().parse::<isize>().unwrap();
    println!("{}", fst - snd);
}
