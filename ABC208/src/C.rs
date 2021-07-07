use std::io::{self, Read};
use std::collections::HashSet;

fn input() -> (usize, usize, Vec<usize>) {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let people_number: Vec<usize> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    (n, k, people_number)
}

fn find_one_more_candy_id(v: &Vec<usize>, bound: usize) -> HashSet<usize> {
    let mut idx = (0..v.len()).collect::<Vec<_>>();
    idx.sort_by(|&i, &j| v[i].cmp(&v[j]));

    let mut one_more_id = HashSet::new();
    for rank in 0..bound {
        one_more_id.insert(idx[rank]);
    }

    one_more_id
}

fn main() {
    let (n, k, mut people_number) = input();

    let all_cnt: usize = k / n;
    let bound = k - all_cnt * n;

    let one_more_id = find_one_more_candy_id(&people_number, bound);
    for idx in 0..n {
        if one_more_id.contains(&idx) {
            println!("{}", all_cnt + 1);
        } else {
            println!("{}", all_cnt);
        }
    }
}