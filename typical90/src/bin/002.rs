use algo::*;

fn dfs(n: usize, score: usize, prefix: String, result: &mut Vec<String>) {
    if n == prefix.len() {
        result.push(prefix);
        return;
    }

    if score < n - prefix.len() {
        dfs(n, score + 1, prefix.clone() + "(", result);
    }
    if score > 0 {
        dfs(n, score - 1, prefix + ")", result);
    }
}

fn solve(n: usize) -> Vec<String> {
    let mut result = vec![];

    if n % 2 == 1 {
        return vec![];
    }

    dfs(n, 0, String::new(), &mut result);

    result
}

fn main() {
    input! {
        n: usize,
    }

    for i in solve(n) {
        println!("{}", i);
    }
}
