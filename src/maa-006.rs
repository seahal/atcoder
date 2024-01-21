// アルゴリズムと数学(ISBN: 978-4297125219)
// 006 - Print 2N+3005 - Modulo 100

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let s: isize = s.trim().parse::<isize>().unwrap();
    println!("{}", s * 2 + 3);
}
