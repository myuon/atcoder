use algo::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn solve(k: usize, mut ax: Vec<i64>, mut by: Vec<i64>, mut cz: Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();

    ax.sort();
    by.sort();
    cz.sort();

    let mut temporal = BinaryHeap::<(i64, usize, usize, usize)>::new();
    temporal.push((
        ax[ax.len() - 1] + by[by.len() - 1] + cz[cz.len() - 1],
        0,
        0,
        0,
    ));
    let mut keys = HashSet::<(usize, usize, usize)>::new();
    keys.insert((0, 0, 0));

    for _ in 0..k {
        let (v, a, b, c) = temporal.pop().unwrap();
        result.push(v);

        if a < ax.len() - 1 && !keys.contains(&(a + 1, b, c)) {
            temporal.push((
                ax[ax.len() - 1 - a - 1] + by[by.len() - 1 - b] + cz[cz.len() - 1 - c],
                a + 1,
                b,
                c,
            ));
            keys.insert((a + 1, b, c));
        }
        if b < by.len() - 1 && !keys.contains(&(a, b + 1, c)) {
            temporal.push((
                ax[ax.len() - 1 - a] + by[by.len() - 1 - b - 1] + cz[cz.len() - 1 - c],
                a,
                b + 1,
                c,
            ));
            keys.insert((a, b + 1, c));
        }
        if c < cz.len() - 1 && !keys.contains(&(a, b, c + 1)) {
            temporal.push((
                ax[ax.len() - 1 - a] + by[by.len() - 1 - b] + cz[cz.len() - 1 - c - 1],
                a,
                b,
                c + 1,
            ));
            keys.insert((a, b, c + 1));
        }
    }

    result
}

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        ax: [i64; x],
        by: [i64; y],
        cz: [i64; z],
    }

    for i in solve(k, ax, by, cz) {
        println!("{}", i);
    }
}
