use std::collections::{BinaryHeap, HashMap};

use algo::*;

fn find_factor(k: usize) -> Option<(usize, usize)> {
    let mut i = 2;
    while i * i <= k {
        if k % i == 0 {
            return Some((i, k / i));
        }

        i += 1;
    }

    None
}

fn get_factors(k: usize) -> BinaryHeap<(usize, usize)> {
    let mut factors = HashMap::new();
    let mut p = k;
    while let Some((i, r)) = find_factor(p) {
        p = r;
        factors.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    factors.entry(p).and_modify(|e| *e += 1).or_insert(1);

    factors.into_iter().collect()
}

fn bfs(mut factors: BinaryHeap<(usize, usize)>, (a, b, c): (usize, usize, usize)) -> usize {
    if factors.is_empty() {
        if a <= b && b <= c {
            return 1;
        } else {
            return 0;
        }
    }

    let mut result = 0;
    let (p, n) = factors.pop().unwrap();
    for i in 0..=n {
        for j in 0..=n {
            if (i + j) > n {
                continue;
            }
            let k = n - i - j;

            result += bfs(
                factors.clone(),
                (
                    a * p.pow(i as u32),
                    b * p.pow(j as u32),
                    c * p.pow(k as u32),
                ),
            );
        }
    }

    result
}

fn solve(k: usize) {
    if k == 1 {
        println!("{}", 1);
    } else {
        let fs = get_factors(k);

        println!("{}", bfs(fs, (1, 1, 1)));
    }
}

fn main() {
    input! {
        k: usize,
    }

    solve(k);
}
