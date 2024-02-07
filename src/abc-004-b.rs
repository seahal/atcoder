// AtCoder Beginner Contest 004
// B - 回転

fn main() {
    let mut matrix = vec![vec![' '; 4]; 4];

    for i in 0..4 {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        let array = buffer
            .trim()
            .split_whitespace()
            .flat_map(|x| x.parse::<char>())
            .collect::<Vec<_>>();

        for (j, c) in array.iter().enumerate() {
            matrix[i][j] = *c;
        }
    }

    let mut matrix2 = matrix.clone();

    for i in 0..=3 {
        for j in 0..=3 {
            matrix2[3 - i][3 - j] = matrix[i][j];
        }
    }

    for (i, m) in matrix2.iter().enumerate() {
        for (j, v) in m.iter().enumerate() {
            print!("{v}");
            if j != 3 {
                print!(" ");
            } else {
                print!("\n");
            }
        }
    }
    //    dbg!(matrix);
}
