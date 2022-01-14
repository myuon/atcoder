use std::collections::{HashMap, HashSet};

use algo::*;

fn find_farest(
    graph: &HashMap<usize, Vec<usize>>,
    current: usize,
    path_len: usize,
    path_set: &mut HashSet<usize>,
) -> (usize, usize) {
    let mut result_dist = 0;
    let mut result_node = 0;

    for &next in graph.get(&current).unwrap() {
        if path_set.contains(&next) {
            continue;
        }

        path_set.insert(next);
        let (dist, node) = find_farest(graph, next, path_len + 1, path_set);
        path_set.remove(&next);

        if dist > result_dist {
            result_dist = dist;
            result_node = node;
        }
    }

    if path_len > result_dist {
        (path_len, current)
    } else {
        (result_dist, result_node)
    }
}

fn solve(n: usize, abs: Vec<(usize, usize)>) -> usize {
    let mut graph = HashMap::new();
    for &(a, b) in &abs {
        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);
    }

    let mut path = HashSet::new();
    path.insert(0);
    let p = find_farest(&graph, 0, 1, &mut path).1;

    let mut path = HashSet::new();
    path.insert(p);
    find_farest(&graph, p, 1, &mut path).0
}

fn main() {
    input! {
        n: usize,
        abs: [(usize, usize); n-1],
    }

    println!(
        "{}",
        solve(n, abs.into_iter().map(|(a, b)| (a - 1, b - 1)).collect())
    );
}
