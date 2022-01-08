use algo::*;

fn solve(n: usize, an: Vec<i64>, bn: Vec<i64>, cn: Vec<i64>) -> i64 {
    let mut dp_a = vec![0; 46];
    for a in an {
        dp_a[a as usize % 46] += 1;
    }

    let mut dp_b = vec![0; 46];
    for b in bn {
        dp_b[b as usize % 46] += 1;
    }

    let mut dp_c = vec![0; 46];
    for c in cn {
        dp_c[c as usize % 46] += 1;
    }

    let mut result = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    result += dp_a[i] * dp_b[j] * dp_c[k];
                }
            }
        }
    }

    result
}

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        bn: [i64; n],
        cn: [i64; n],
    }

    println!("{}", solve(n, an, bn, cn));
}
