// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A06 - How Many Guests?
use std::io::BufRead;

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let ax = s
        .split_whitespace()
        .flat_map(|i| i.parse::<usize>())
        .collect::<Vec<usize>>();

    let mut sx: Vec<usize> = Vec::new();

    for i in 0..ax.len() {
        let mut j: usize = 0;
        for k in 0..=i {
            j += ax[k];
        }
        sx.push(j);
    }

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    for j in stdin.lines() {
        let l_and_r = j.unwrap().clone();
        let mut lr = l_and_r.split(" ").flat_map(|n| n.parse::<usize>());
        //            ;
        let l = lr.nth(0).unwrap();
        let r = lr.nth(0).unwrap();

        let la = if l < 2 { 0 } else { sx[l - 2] };
        let ra = sx[r - 1];
        println!("{}", ra - la);
    }
}
