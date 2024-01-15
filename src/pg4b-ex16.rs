// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX16 - 隣り合う同じ値を探す

fn main() {
    let mut s = String::new();
    let _r = std::io::stdin().read_line(&mut s);
    let w = s
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|v| v.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let x = w.windows(2);

    for i in x {
        if i[0] == i[1] {
            println!("YES");
            // std::process::exit(0);
            return ();
        }
    }

    println!("NO");
}
