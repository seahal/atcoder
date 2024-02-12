// AtCoder Beginner Contest(ABC), 017
// A - プロコン

fn main() {
    let ans = std::io::stdin().lines().map(|str| {
        let md = str
            .unwrap()
            .trim()
            .split_whitespace()
            .flat_map(|x| x.parse::<f64>())
            .collect::<Vec<_>>();
        md[0] * md[1] * 0.1
    });
    println!("{}", ans.sum::<f64>());
}
