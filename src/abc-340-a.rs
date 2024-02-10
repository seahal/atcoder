// AtCoder Beginner Contest 340
//  A - Arithmetic Progression /

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut array = buffer.split_whitespace().flat_map(|x| x.parse::<usize>());

    let a = array.next().unwrap();
    let b = array.next().unwrap();
    let n = array.next().unwrap();

    let ans = (a..=b)
        .step_by(n)
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{ans}");
}
