// 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
// B03, Supermarket 1

fn main() {
    proconio::input! {
        n: u32,
        a: [u32; n],
    }

    for i in &a {
        for j in &a {
            for k in &a {
                if i != j && k != j && k != i && i + j + k == 1_000u32 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
