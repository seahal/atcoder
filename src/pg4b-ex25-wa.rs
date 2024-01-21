// EX25 - 集合の操作

use std::{collections::BTreeSet, ops::Sub};

fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let a: BTreeSet<isize> = s
        .split(char::is_whitespace)
        .flat_map(|v| v.parse::<isize>())
        .collect();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let b: BTreeSet<isize> = s
        .split(char::is_whitespace)
        .flat_map(|v| v.parse::<isize>())
        .collect();

    let c = b.intersection(&a);
    let c = b.union(&a);
    let c = b.symmetric_difference(&a);
    let c = a.difference(&b);

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let com = s.trim();

    if (com == "intersection") {
        println!(
            "{}",
            a.intersection(&b)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else if (com == "union_set") {
        println!(
            "{}",
            a.union(&b)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else if (com == "symmetric_diff") {
        println!(
            "{}",
            b.symmetric_difference(&a)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else if (com == "subtract") {
        println!(
            "{}",
            a.symmetric_difference(&b)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else if (com == "increment") {
        println!(
            "{}",
            a.iter()
                .map(|v| if v == &49isize {
                    "0".to_owned()
                } else {
                    (v + 1).to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else if (com == "decrement") {
        println!(
            "{}",
            a.iter()
                .map(|v| if v == &0isize {
                    "49".to_owned()
                } else {
                    (v - 1).to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
