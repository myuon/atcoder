use algo::*;

fn solve(n: usize, w: usize, wvs: Vec<[i32; 2]>) -> i32 {
    let mut wc = vec![vec![]; 4];
    for [w, v] in wvs.clone() {
        wc[(w as i32 - wvs[0][0]) as usize].push(v);
    }

    for i in 0..4 {
        wc[i].sort_by(|v1, v2| v2.cmp(v1));
    }

    let mut r = 0;
    let w1 = wvs[0][0];
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k > n {
                    continue;
                }

                for l in 0..=n - (i + j + k) {
                    if w1 as usize * i
                        + (w1 as usize + 1) * j
                        + (w1 as usize + 2) * k
                        + (w1 as usize + 3) * l
                        > w
                    {
                        continue;
                    }
                    let p = wc[0].iter().take(i).sum::<i32>()
                        + wc[1].iter().take(j).sum::<i32>()
                        + wc[2].iter().take(k).sum::<i32>()
                        + wc[3].iter().take(l).sum::<i32>();
                    if r < p {
                        r = p;
                    }
                }
            }
        }
    }

    r
}

fn main() {
    input! {
        n: usize,
        w: usize,
        wvs: [[i32; 2]; n],
    }

    println!(
        "{:?}",
        solve(n, w, wvs.into_iter().map(|v| [v[0], v[1]]).collect())
    );
}
