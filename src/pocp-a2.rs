// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// A02, Linear Search

fn main() {
    proconio::input! {
    n: i8,
    x: i8,
    a: [i8; n]
    }

    let mut z = false;

    for i in a {
        if i == x {
            z = true;
            break;
        }
    }

    println!("{}", if z { "Yes" } else { "No" });
}
