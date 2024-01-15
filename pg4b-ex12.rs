// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX12 - 足したり引いたり

fn main() {
    let mut s = String::new();
    let r = std::io::stdin().read_line(&mut s);
    let mut count:isize  = 1;

    for i in s.chars() {
        match i {
            '+' => count += 1,
            '-' => count -= 1,
            _ => (),
        }
    }
    println!("{count}");
}
