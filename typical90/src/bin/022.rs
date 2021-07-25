use algo::*;
use num::integer::gcd;

fn solve(a: i64, b: i64, c: i64) -> i64 {
    let l = gcd(gcd(a, b), c);

    (a / l - 1) + (b / l - 1) + (c / l - 1)
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    println!("{}", solve(a, b, c));
}
