// 問題解決のための「アルゴリズム×数学」が基礎からしっかり身につく本(ISBN: 978-4297125219)
// 086 - Parentheses Check

fn main() -> () {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n: isize = buffer.trim().parse().unwrap();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut array: Vec<String> = buffer
        .trim()
        .matches(&['(', ')'])
        .map(|x| x.to_string())
        .collect();
    let mut depth = 0;
    let mut result = true;

    for i in array {
        match i.as_str() {
            "(" => depth += 1,
            ")" => depth -= 1,
            _ => panic!("nothing"),
        }
        if depth < 0 {
            result = false;
            break;
        }
    }

    if depth == 0 {
        result = true;
    }

    println!("{}", if result { "Yes" } else { "No" });
}
