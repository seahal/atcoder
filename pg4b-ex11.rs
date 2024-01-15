// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX11 - 電卓をつくろう2

fn main() {
    let mut s = Default::default();
    let _ = std::io::stdin().read_line(&mut s);
    let _ = s.trim().parse::<isize>().unwrap();

    s.clear();
    let _ = std::io::stdin().read_line(&mut s);
    let mut number: isize = s.trim().parse::<isize>().unwrap();
    let mut count: isize = 1;

    loop {
        match get_wild() {
            Some((c, n)) => {
                let result = match c {
                    '+' => {
                        number = number + n;
                        format!("{count}:{number}")
                    }
                    '-' => {
                        number = number - n;
                        format!("{count}:{number}")
                    }
                    '*' => {
                        number = number * n;
                        format!("{count}:{number}")
                    }
                    '/' if n == 0 => {
                        format!("error")
                    }
                    '/' => {
                        number = number / n;
                        format!("{count}:{number}")
                    }
                    _ => format!("error"),
                };

                println!("{result}");
                if result == format!("error") {
                    break;
                }
                count += 1;
            }
            None => break,
        }
    }
}

fn get_wild() -> Option<(char, isize)> {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut i = s.trim().split_whitespace().into_iter();
    let exp = i.nth(0);
    let num = i.last();

    if let None = exp {
        return None;
    }

    Some((
        exp.unwrap().parse::<char>().unwrap(),
        num.unwrap().parse::<isize>().unwrap(),
    ))
}
