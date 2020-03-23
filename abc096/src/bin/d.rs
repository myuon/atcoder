use algo::*;

use std::collections::{HashMap, HashSet};
fn prime_sieves(n: usize) -> HashSet<usize> {
    let mut sieve = HashMap::new();
    for i in 2..n {
        sieve.insert(i, true);
    }

    let mut primes = HashSet::new();
    for i in 2..n {
        if !sieve[&i] {
            continue;
        }

        primes.insert(i);

        let mut j = i;
        while j <= n {
            sieve.insert(j, false);
            j += i
        }
    }

    primes
}

fn solve(n: usize) -> Vec<usize> {
    let primes = prime_sieves(55555);

    primes
        .into_iter()
        .filter(|v| v % 5 == 1)
        .take(n)
        .collect::<Vec<_>>()
}

fn main() {
    input! {
        n: usize
    }

    println!(
        "{}",
        solve(n)
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
