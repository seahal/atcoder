// AtCoder Beginner Contest(ABC), 022
// A - Best Body

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let num: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<isize>())
        .collect();

    let s = num[1];
    let t = num[2];

    let mut body_weight = 0isize;
    let mut count = 0;

    for i in std::io::stdin().lines() {
        let n = i.unwrap().trim().parse::<isize>().unwrap();
        body_weight += n;
        if (s..=t).contains(&body_weight) {
            count += 1;
        }
    }

    println!("{count}");
}
