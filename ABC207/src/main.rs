use std::io;

fn input() -> Vec<usize>{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let a: Vec<usize> = buffer.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a
}

fn main() {
    let a = input();
    let total: usize = a.iter().sum();
    let mut result = 0;

    for i in a {
        if result <= total - i {
            result = total - i;
        }
    }

    // add other answer
    // use std::cmp::min;
    // let result = a.iter().sum() - min(min(a,b),c);

    println!("{}", result);
}
