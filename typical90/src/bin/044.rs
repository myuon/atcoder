use algo::*;

fn solve(n: usize, q: usize, mut an: Vec<i64>, txy: Vec<(usize, usize, usize)>) -> Vec<i64> {
    let mut index = 0;
    let mut ans = vec![];
    for i in 0..q {
        let (t, x, y) = txy[i];
        if t == 1 {
            an.swap((x + n - index + n - 1) % n, (y + n - index + n - 1) % n);
        } else if t == 2 {
            index = (index + 1) % n;
        } else if t == 3 {
            ans.push(an[(x + n - index + n - 1) % n]);
        }
    }

    ans
}

fn main() {
    input! {
        n: usize,
        q: usize,
        an: [i64; n],
        txy: [(usize, usize, usize); q],
    }

    for i in solve(n, q, an, txy) {
        println!("{}", i);
    }
}
