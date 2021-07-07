use std::io::{self, Read};
const INF: usize = 999999999999999;

fn input() -> (usize, usize, Vec<Vec<usize>>) {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<Vec<usize>> = vec![vec![0; n]; n];

    for idx in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        v[a-1][b-1] = c;
    }

    (n, m, v)
}

fn main() {
    let (n, m, v) = input();

    let mut ans: Vec<Vec<usize>> = vec![vec![INF; n]; n];
    for x in 0..n {
        for (i, y) in v[x].iter().enumerate() {
            if v[x][i] != 0 {
                ans[x][i] = *y;
            }
        }
    }
    for x in 0..n {
        ans[x][x] = 0;
    }

    let mut final_ans = 0;

    for k in 0..n {
        for x in 0..n {
            for y in 0..n {
                ans[x][y] = std::cmp::min(ans[x][y], ans[x][k] + ans[k][y]);
                if ans[x][y] != INF {
                    final_ans += ans[x][y];
                }
            }
        }
    }
    println!("{}", final_ans);
}