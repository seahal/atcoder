// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A11 - Binary Search 1, 復習, TLE

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut o = s.split_whitespace();
    let n = o.next().unwrap().parse::<usize>().unwrap();
    let x = o.next().unwrap().parse::<usize>().unwrap();

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let a = s
        .split_whitespace()
        .flat_map(|o| o.parse::<usize>())
        .collect::<Vec<_>>();

    let mut left: usize = 0;
    let mut right: usize = n - 1;
    let mut result: usize = 1;

    while left < right {
        let m = (left + right) / 2;
        if a[m] == x {
            result += m;
            break;
        } else if a[m] < x {
            left = m - 1;
        } else if x < a[m] {
            right = m + 1;
        }
    }

    println!("{result}");
}
