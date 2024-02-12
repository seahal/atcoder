// AtCoder Beginner Contest(ABC), 018
// A - 豆まき

fn main() {
    let mut array = std::io::stdin()
        .lines()
        .flat_map(|x| x.unwrap().parse::<usize>())
        .collect::<Vec<_>>();

    for &a in &array {
        println!("{}", &array.iter().filter(|&&x| x > a).count() + 1);
    }
}
