use algo::*;

fn solve(n: usize, an: Vec<i64>, q: usize, bn: Vec<i64>) -> Vec<i64> {
    let mut result = vec![];
    let mut cs = an;
    cs.sort();

    for b in bn {
        let i = cs.binary_search(&b).unwrap_or_else(|t| t);
        result.push(if i == 0 {
            cs[i] - b
        } else if i == n {
            b - cs[i - 1]
        } else {
            (cs[i] - b).min(b - cs[i - 1])
        });
    }

    result
}

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        q: usize,
        bn: [i64; q],
    }

    for b in solve(n, an, q, bn) {
        println!("{}", b);
    }
}
