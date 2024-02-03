// AtCoder Beginner Contest 339
// C - Perfect Bus

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let n = s.trim().parse::<usize>().unwrap();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let list = s
        .trim()
        .split(" ")
        .flat_map(|x| x.parse::<isize>())
        .collect::<Vec<_>>();

    let mut diff: Vec<isize> = vec![];
    for i in 0..=n {
        diff.push(list[0..i].iter().sum::<isize>());
    }

    let mut iter = diff.iter();
    let last = iter.clone().last();

    let mut additonal: isize = 0;
    let _ = iter.next();
    if let Some(&x) = iter.by_ref().min() {
        if x < 0 {
            additonal = x * -1;
        } else {
            additonal = 0;
        }
    };

    println!("{:}", last.unwrap() + additonal);
}
