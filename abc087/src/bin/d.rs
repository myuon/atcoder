use algo::*;
use std::collections::HashMap;

fn solve(n: usize, m: usize, lrds: Vec<[i32; 3]>) -> bool {
    let mut indices = HashMap::new();
    for [l, r, _] in &lrds {
        indices.insert(l, 0);
        indices.insert(r, 0);
    }

    let indices_clone = indices.clone();
    let mut xs = indices_clone.keys().collect::<Vec<_>>();
    xs.sort();

    for i in 0..xs.len() {
        indices.insert(xs[i], i);
    }

    let mut dists = HashMap::new();
    let mut graph = vec![Vec::new(); n];
    let mut weights = vec![0; n];

    for [l, r, d] in &lrds {
        dists.insert((indices[&l], indices[&r]), *d);
        dists.insert((indices[&r], indices[&l]), -d);

        graph[indices[&l]].push(indices[&r]);
        graph[indices[&r]].push(indices[&l]);
    }

    weights[0] = 0;

    let mut visited = vec![false; n];

    fn rec(
        graph: &Vec<Vec<usize>>,
        dists: &HashMap<(usize, usize), i32>,
        weights: &mut Vec<i32>,
        visited: &mut Vec<bool>,
        e: usize,
    ) -> bool {
        visited[e] = true;

        for t in &graph[e] {
            if visited[*t] {
                if weights[e] + dists[&(e, *t)] != weights[*t] {
                    return false;
                }
            } else {
                weights[*t] = weights[e] + dists[&(e, *t)];

                if !rec(graph, dists, weights, visited, *t) {
                    return false;
                }
            }
        }

        true
    }

    for i in 0..n {
        if visited[i] {
            continue;
        }

        if !rec(&graph, &dists, &mut weights, &mut visited, 0) {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lrds: [[i32; 3]; m],
    }

    println!(
        "{}",
        if solve(
            n,
            m,
            lrds.into_iter()
                .map(|v| [v[0] - 1, v[1] - 1, v[2]])
                .collect()
        ) {
            "Yes"
        } else {
            "No"
        }
    );
}
