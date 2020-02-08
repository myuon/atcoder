use algo::*;

fn solve(n: usize, s: String) -> Vec<bool> {
    let input = s.chars().map(|v| v == 'o').collect::<Vec<_>>();

    for [s0, s1] in [[true, true], [true, false], [false, true], [false, false]].iter() {
        let mut sheeps = vec![false; n];
        sheeps[0] = *s0;
        sheeps[1] = *s1;

        for i in 2..n {
            if sheeps[i - 1] && input[i - 1] {
                sheeps[i] = sheeps[i - 2];
            } else if sheeps[i - 1] && !input[i - 1] {
                sheeps[i] = !sheeps[i - 2];
            } else if !sheeps[i - 1] && input[i - 1] {
                sheeps[i] = !sheeps[i - 2];
            } else {
                sheeps[i] = sheeps[i - 2];
            }
        }

        if sheeps[0] && input[0] && sheeps[n - 1] != sheeps[1] {
            continue;
        }
        if sheeps[0] && !input[0] && sheeps[n - 1] == sheeps[1] {
            continue;
        }
        if !sheeps[0] && input[0] && sheeps[n - 1] == sheeps[1] {
            continue;
        }
        if !sheeps[0] && !input[0] && sheeps[n - 1] != sheeps[1] {
            continue;
        }

        if sheeps[n - 1] && input[n - 1] && sheeps[n - 2] != sheeps[0] {
            continue;
        }
        if sheeps[n - 1] && !input[n - 1] && sheeps[n - 2] == sheeps[0] {
            continue;
        }
        if !sheeps[n - 1] && input[n - 1] && sheeps[n - 2] == sheeps[0] {
            continue;
        }
        if !sheeps[n - 1] && !input[n - 1] && sheeps[n - 2] != sheeps[0] {
            continue;
        }

        return sheeps;
    }

    return Vec::new();
}

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let rs = solve(n, s);
    if rs.is_empty() {
        println!("-1");
    } else {
        use std::iter::FromIterator;

        println!(
            "{}",
            String::from_iter(rs.into_iter().map(|b| if b { 'S' } else { 'W' }))
        );
    }
}
