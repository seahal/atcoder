// AtCoder Beginner Contest (ABC), 002
// B - 罠

fn main() {
    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);
    let array = buff
        .trim()
        .split(&['a', 'e', 'i', 'o', 'u'])
        .collect::<Vec<_>>()
        .join("");

    println!("{array}");
}
