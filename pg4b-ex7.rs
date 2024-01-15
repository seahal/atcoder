// C++入門 AtCoder Programming Guide for beginners (APG4b)
// EX7 - bool値パズル

fn main() {
    // (!)spoiler
    // println!("AtCoder");

    let mut cout = String::new();
    let a: bool = true; // true or false
    let b: bool = false; // true or false
    let c: bool = true; // true or false

    if a {
        cout += "At";
    } else {
        cout += "Yo";
    }

    if (!a && b) {
        cout += "Bo";
    } else if (!b || c) {
        cout += "Co";
    }

    if (a && b && c) {
        cout += "foo!";
    } else if (true && false) {
        cout += "yeah!";
    } else if (!a || c) {
        cout += "der";
    }

    println!("{cout}");
}
