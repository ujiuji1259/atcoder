
fn input() -> (usize, usize, usize, usize) {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let A: usize = iter.next().unwrap().parse().unwrap();
    let B: usize = iter.next().unwrap().parse().unwrap();
    let C: usize = iter.next().unwrap().parse().unwrap();
    let D: usize = iter.next().unwrap().parse().unwrap();

    (A, B, C, D)
}

fn main() {
    let (A, B, C, D) = input();
    if D * C <= B {
        println!("{}", -1);
    } else {
        // A + n*B <= D * n * C
        // A <= n * (DC - B)
        // A / (DC - B) >= n
        let diff = D * C - B;
        let result: usize =  (A + diff - 1) / diff;
        println!("{}", result);
    }
}