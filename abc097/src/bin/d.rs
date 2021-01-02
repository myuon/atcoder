use std::collections::{HashMap, HashSet};

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

pub struct UGraph(usize, Vec<Vec<usize>>);

impl UGraph {
    pub fn new_from_edges(size: usize, edges: Vec<(usize, usize)>) -> UGraph {
        let mut nodes = vec![vec![]; size];
        for (x, y) in edges {
            nodes[x].push(y);
            nodes[y].push(x);
        }

        UGraph(size, nodes)
    }

    pub fn to_grouped(self) -> Vec<Vec<usize>> {
        let UGraph(size, node) = self;

        let mut groups = vec![];
        let mut visited = vec![false; size];

        for i in 0..size {
            if visited[i] {
                continue;
            }

            let mut current = vec![];
            current.push(i);

            let mut to_visit = node[i].clone();
            while let Some(p) = to_visit.pop() {
                if visited[p] {
                    continue;
                }

                current.push(p);
                visited[p] = true;

                let mut n = node[p].clone();
                to_visit.append(&mut n);
            }

            groups.push(current);
        }

        groups
    }
}

fn solve(n: usize, m: usize, ps: Vec<i32>, xys: Vec<(usize, usize)>) -> i32 {
    let groups = UGraph::new_from_edges(n, xys).to_grouped();
    let ps_indices = ps
        .iter()
        .enumerate()
        .map(|(x, y)| (*y, x))
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    for group in groups {
        let p = group
            .iter()
            .map(|i| ps_indices[&(*i as i32)])
            .collect::<HashSet<_>>();
        let g = group.into_iter().collect::<HashSet<_>>();
        count += p.intersection(&g).count();
    }

    count as i32
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [i32; n],
        xys: [[usize; 2]; m],
    }

    println!(
        "{}",
        solve(
            n,
            m,
            ps.into_iter().map(|p| p - 1).collect(),
            xys.into_iter().map(|v| (v[0] - 1, v[1] - 1)).collect()
        )
    );
}
