// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A01, The First Problem, Re

fn main() {
    let mut s: String = "".into();
    let _ = std::io::stdin().read_line(&mut s);
    let v = s.trim().parse::<i32>().unwrap_or(0);
    println!("{}", v.pow(2));
}
