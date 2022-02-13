use algo::*;

fn solve(h: usize, w: usize, mut an: Vec<Vec<i64>>, bn: Vec<Vec<i64>>) {
    let mut result = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let c = bn[i][j] - an[i][j];

            result += c.abs();
            an[i][j] += c;
            an[i + 1][j] += c;
            an[i][j + 1] += c;
            an[i + 1][j + 1] += c;
        }
    }

    for i in 0..h {
        for j in 0..w {
            if an[i][j] != bn[i][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes\n{}", result);
}

fn main() {
    input! {
        h: usize,
        w: usize,
        an: [[i64; w]; h],
        bn: [[i64; w]; h],
    }

    solve(h, w, an, bn);
}
