use algo::*;

fn solve(n: usize, c: i64, xvs: Vec<[i64; 2]>) -> i64 {
    // 時計回り
    let mut val_r = vec![(0, 0); n];
    let mut acc_r = vec![(0, 0); n];
    // 反時計周り
    let mut val_l = vec![(0, 0); n];
    let mut acc_l = vec![(0, 0); n];

    for i in 0..n {
        let [x, v] = xvs[i];
        if i == 0 {
            val_r[i] = (x, v - x);
        } else {
            let (px, pv) = val_r[i - 1];
            val_r[i] = (x, pv + v - (x - px));
        }
    }

    for i in 0..n {
        let [y, v] = xvs[n - 1 - i];
        let x = c - y;

        if i == 0 {
            val_l[i] = (x, v - x);
        } else {
            let (px, pv) = val_l[i - 1];
            val_l[i] = (x, pv + v - (x - px));
        }
    }

    let mut max = (0, 0);
    for i in 0..n {
        if val_l[i].1 > max.1 {
            max = val_l[i];
        }

        acc_l[i] = max;
    }

    let mut max = (0, 0);
    for i in 0..n {
        if val_r[i].1 > max.1 {
            max = val_r[i];
        }

        acc_r[i] = max;
    }

    let mut r = 0;
    for i in -1..(n as i32) {
        let (x_l, v_l) = if i == -1 { (0, 0) } else { acc_l[i as usize] };
        let (x_r, v_r) = if i == n as i32 - 1 {
            (0, 0)
        } else {
            acc_r[(n as i32 - i - 2) as usize]
        };

        // 両方
        let r_both = v_l + v_r - x_l.min(x_r);
        let ri = vec![r_both, v_l, v_r].into_iter().max().unwrap();
        if ri > r {
            r = ri;
        }
    }

    r
}

fn main() {
    input! {
        n: usize,
        c: i64,
        xvs: [[i64; 2]; n],
    }

    println!(
        "{:?}",
        solve(n, c, xvs.into_iter().map(|v| [v[0], v[1]]).collect())
    );
}
