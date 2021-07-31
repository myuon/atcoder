use algo::*;

fn solve(n: usize, m: usize, abs: Vec<(i32, i32)>) -> i32 {
    let mut graph = vec![vec![]; n];

    for (a, b) in abs {
        graph[a.max(b) as usize - 1].push(a.min(b) - 1);
    }

    let mut c = 0;
    for i in 0..n {
        if graph[i].len() == 1 {
            c += 1;
        }
    }

    c
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(i32, i32); m]
    }

    println!("{}", solve(n, m, abs));
}
