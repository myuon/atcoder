use algo::*;

fn solve(n: usize, k: usize, an: Vec<i64>) -> i64 {
    let mut count = 0;

    // 現在の区間合計
    let mut s = 0;
    let mut r = 0;

    'search: for l in 0..n {
        // lから始まる最小区間
        while s < k as i64 {
            if r == n {
                break 'search;
            }

            s += an[r];
            r += 1;
        }

        count += n as i64 - r as i64 + 1;
        s -= an[l];
    }

    count
}

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }

    println!("{:?}", solve(n, k, an));
}
