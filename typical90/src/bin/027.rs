use std::collections::HashSet;

use algo::*;

fn solve(n: usize, sn: Vec<String>) -> Vec<usize> {
    let mut cache = HashSet::new();
    let mut result = Vec::new();

    for i in 0..n {
        let si = &sn[i];
        if cache.contains(si) {
            continue;
        }

        result.push(i);
        cache.insert(si.clone());
    }

    result
}

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let result = solve(n, sn);
    for i in result {
        println!("{}", i + 1);
    }
}
