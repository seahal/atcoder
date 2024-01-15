// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX17 - 果物屋さんでお買い物

fn main() {
    let mut count: isize = 0;

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let r = s
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<isize>()
        .unwrap();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let a = s.split_whitespace().map(|n| n.parse::<isize>().unwrap());

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let p = s.split_whitespace().map(|n| n.parse::<isize>().unwrap());

    for i in a {
        for j in p.clone() {
                if i + j == r {
                    count+=1;
                }
        }
    }

    println!("{}", count);
}
