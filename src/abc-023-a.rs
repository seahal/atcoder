// AtCoder Beginner Contest(ABC), 021
// A - 加算王, https://atcoder.jp/contests/abc021/tasks/abc021_a

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let ans: Vec<_> = buffer
        .trim()
        .matches(char::is_numeric)
        .flat_map(|x| x.parse::<isize>())
        .collect();
    println!("{}", ans[0] + ans[1]);
}
