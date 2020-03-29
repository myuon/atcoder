use algo::*;
use std::collections::HashMap;

fn solve(n: usize, an: Vec<i32>) -> usize {
    let mut count = HashMap::new();
    for a in an {
        *count.entry(a).or_insert(0) += 1;
    }

    let k = count.keys().len();

    k - if (n - k) % 2 == 0 { 0 } else { 1 }
}

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }

    println!("{:?}", solve(n, an));
}
