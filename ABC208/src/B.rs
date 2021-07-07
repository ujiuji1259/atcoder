
fn input() -> usize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let p: usize = iter.next().unwrap().parse().unwrap();
    p
}

fn main() {
    let mut p = input();

    let mut v = vec![1; 10];
    for i in 1..10 {
        v[i] = v[i-1] * (i+1);
    }
    v.reverse();

    let mut ans = 0;
    for pow in &v {
        let div: usize = std::cmp::min(p / pow, 100);
        if div > 0 {
            ans += div;
            p -= div * pow;
        }
    }
    println!("{}", ans);
}