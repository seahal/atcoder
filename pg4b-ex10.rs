// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX10 - 棒グラフの出力

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut i = s.split_whitespace().into_iter();

    let a = changer(&mut i);
    let b = changer(&mut i);
    println!("A:{}", count_up(a));
    println!("B:{}", count_up(b));
}

fn count_up(n: isize) -> String {
    let mut s = String::new();
    for _ in 1..=n {
        s += "]";
    }
    s
}

fn changer(i: &mut std::str::SplitWhitespace) -> isize {
    i.next().unwrap().parse::<isize>().unwrap()
}
