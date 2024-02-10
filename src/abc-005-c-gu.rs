// AtCoder Beginner Contest 005
// C - おいしいたこ焼きの作り方

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let t = buffer.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let ax = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let m = buffer.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let bx = buffer
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();

    let mut answer = ax
        .iter()
        .zip(bx)
        .take(n.min(m))
        .all(|(a, b)| a < &b && &b < &(a + t));

    if n < m {
        answer = false;
    }
    println!("{}", if answer { "yes" } else { "no" });
}
