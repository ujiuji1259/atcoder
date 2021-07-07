use std::io::{self, Read};

fn input() -> (usize, usize) {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn main() {
    let (a, b) = input();

    if a <= b && b <= 6 * a {
        println!("Yes");
    } else {
        println!("No");
    }
}
