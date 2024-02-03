// AtCoder Beginner Contest 339
// B - Langton's Takahashi

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let list = s
        .trim()
        .split(" ")
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();

    let h = list[0];
    let w = list[1];
    let n = list[2];

    let grid = vec![vec![false; w]; h];
    let mut pointer = (0usize, 0usize);
    let mut work: Vec<(usize, usize)> = vec![];

    for i in 0..n {}

    for i in 0..h {
        for j in 0..w {
            print!("{}", if grid[i][j] { "#" } else { "." });
        }
        println!("");
    }
}
