use std::collections::BinaryHeap;

use algo::*;

fn solve(n: usize, k: usize, abn: Vec<(usize, usize)>) -> i64 {
    let mut result = 0;
    let mut scores = BinaryHeap::new();
    for (i, (a, b)) in abn.iter().enumerate().collect::<Vec<_>>() {
        scores.push((*b, i, 1));
    }

    for _ in 0..k {
        let (p, i, k) = scores.pop().unwrap();
        if k == 1 {
            scores.push((abn[i].0 - abn[i].1, i, 2));
        }
        result += p as i64;
    }

    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        abn: [(usize, usize); n],
    }

    println!("{}", solve(n, k, abn));
}
