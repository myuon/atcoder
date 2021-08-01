use itertools::*;
use proconio::*;

fn main() {
    input! {
        n: usize,
        aij: [[i64; n]; n],
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (x, y) = xy[i];
        graph[x - 1].push(y - 1);
        graph[y - 1].push(x - 1);
    }

    let inf = 1_000_000_000;
    let mut min = inf;
    for seq in (0..n).permutations(n) {
        let mut possible = true;
        for i in 0..n - 1 {
            if graph[seq[i]].contains(&seq[i + 1]) {
                possible = false;
                break;
            }
        }

        if !possible {
            continue;
        }

        let mut cost = 0;
        for i in 0..n {
            cost += aij[seq[i]][i];
        }

        min = min.min(cost);
    }

    println!("{}", if min == inf { -1 } else { min });
}
