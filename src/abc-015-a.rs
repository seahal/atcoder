// AtCoder Beginner Contest(ABC), 015
// A - 高橋くんの研修

fn main() {
    let mut buffer = String::new();
    let mut x = std::io::stdin().lines().flat_map(|x| x).collect::<Vec<_>>();
    x.sort_by_key(|x| x.len());
    println!("{:}", x.last().unwrap());
}
