// AtCoder Regular Contest 170
// A - Yet Another AB Problem
// WA
fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let n = s
        .trim()
        .split(' ')
        .nth(0)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let upper: Vec<&str> = s.trim().matches(char::is_alphabetic).collect();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let lower: Vec<&str> = s.trim().matches(char::is_alphabetic).collect();

    let mut up = upper.clone();
    for count in (1..=n).rev() {
        for k in 0..n {
            for l in k + 1..n {
                up = change(up.clone(), k, l);
                let result = up == lower;

                if result == true {
                    println!("{}", count);
                    return ();
                }
            }
        }
    }

    println!("{}", -1);
}

fn change(v: Vec<&str>, i: usize, j: usize) -> Vec<&str> {
    let mut w = v.clone();
    w[i] = "A";
    w[j] = "B";
    w
}
