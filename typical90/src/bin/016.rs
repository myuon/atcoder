use proconio::*;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let max_sum = 10000;
    let mut m = 1_000_000;
    for x in 0..max_sum {
        if a * x > n {
            continue;
        }

        for y in 0..(max_sum - x) {
            if a * x + b * y > n {
                continue;
            }

            if (n - (a * x + b * y)) % c == 0 {
                m = m.min(x + y + (n - (a * x + b * y)) / c);
            }
        }
    }

    println!("{}", m);
}
