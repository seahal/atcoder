// AtCoder Beginner Contest 338
// A - Capitalized?

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut array = s.trim().chars();
    let first = array.next().unwrap();
    let one = first.to_uppercase().to_string() == first.to_string();

    let mut reslut = false;
    let res = array.all(|c| c.to_lowercase().to_string() == c.to_string());

    println!("{}", if res && one { "Yes" } else { "No" });
}
