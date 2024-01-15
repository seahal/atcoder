// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX8 - たこ焼きセット

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let pattern = s.trim().parse::<isize>().unwrap();

    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let two = s.trim();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let three = s.trim();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let four = s.trim();

    let prev: isize;
    let next: isize;

    match pattern {
        1 => {
            prev = two.parse::<isize>().unwrap();
            next = three.parse::<isize>().unwrap();
            println!("{}", prev * next);
        }
        2 => {
            println!("{}!", two);
            prev = three.parse::<isize>().unwrap();
            next = four.parse::<isize>().unwrap();
            println!("{}", prev * next);
        }
        _ => {}
    }

}
