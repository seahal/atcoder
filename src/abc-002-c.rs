// AtCoder Beginner Contest (ABC), 002
// C - 直訴

fn main() {
    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);
    let binding = buff
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<f64>())
        .collect::<Vec<_>>();
    let mut binding = binding.chunks(2).collect::<Vec<_>>();

    binding.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut a0 = binding[0][0];
    let mut a1 = binding[0][1];
    let mut b0 = binding[1][0] - a0;
    let mut b1 = binding[1][1] - a1;
    let mut c0 = binding[2][0] - a0;
    let mut c1 = binding[2][1] - a1;

    println!("{}", (b0 * c1 - b1 * c0).abs() / 2.0);
}
