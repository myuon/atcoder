use algo::*;

fn solve(h: usize, w: usize, aij: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut pre_rows = vec![0; h];
    let mut pre_columns = vec![0; w];
    for i in 0..w {
        for j in 0..h {
            pre_rows[j] += aij[j][i];
            pre_columns[i] += aij[j][i];
        }
    }

    let mut bij = vec![vec![0; w]; h];
    for i in 0..w {
        for j in 0..h {
            bij[j][i] = pre_rows[j] + pre_columns[i] - aij[j][i];
        }
    }

    bij
}

fn main() {
    input! {
        h: usize,
        w:usize,
        aij: [[usize; w];h]
    }

    let bij = solve(h, w, aij);
    for bj in bij {
        println!(
            "{}",
            bj.into_iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
