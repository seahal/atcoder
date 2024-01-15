// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX9 - 複合代入演算子を使おう

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut i = s.trim().split_whitespace().into_iter();

    let x: isize = i.next().unwrap().parse::<isize>().unwrap();
    let a: isize = i.next().unwrap().parse::<isize>().unwrap();
    let b: isize = i.next().unwrap().parse::<isize>().unwrap();

    let one = x + 1;
    println!("{}", one);
    let two = (a + b) * one;
    println!("{}", two);
    let three = two * two;
    println!("{}", three);
    let four = three - 1;
    println!("{}", four);
}
