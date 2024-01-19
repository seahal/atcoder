// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A11 - Binary Search 1

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut o = s.split_whitespace().collect::<Vec<_>>();
    let N = o.first().unwrap().parse::<usize>().unwrap();
    let X = o.last().unwrap().parse::<usize>().unwrap();

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let mut A = s
        .split_whitespace()
        .flat_map(|o| o.parse::<usize>())
        .collect::<Vec<_>>();

    let mut L: usize = 0;
    let mut R: usize = N-1;
    let mut result: usize = 0;

    while L <= R {
        let m = (L + R) / 2;
        if X < A[m] {
            R = m - 1;
        } else if X > A[m] {
            L = m + 1;
        } else {
            result = m;
            break;
        }
    }

    println!("{}", result + 1);
}
