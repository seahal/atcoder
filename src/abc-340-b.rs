// AtCoder Beginner Contest 340
// B - Append

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut v: Vec<usize> = Vec::new();

    for i in 1..=n {
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
        let mut array = buffer.split_whitespace().flat_map(|x| x.parse::<usize>());

        let fst = array.next().unwrap();
        let snd = array.next().unwrap();

        match fst {
            1 => v.push(snd),
            2 => println!("{}", v[v.len() - snd]),
            _ => panic!(),
        }
    }
}
