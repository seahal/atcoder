// AtCoder Beginner Contest 003
// C - AtCoderプログラミング講座

fn main() {
    let score: f64 = 0.0;

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let nk = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();
    let _n = nk[0];
    let k = nk[1];

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut array = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<f64>())
        .collect::<Vec<_>>();
    array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!(
        "{}",
        array
            .iter()
            .rev()
            .take(k)
            .rev()
            .fold(score, |acc, num| (acc + num) / 2.0)
    );
}
