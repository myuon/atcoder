use num::Integer;
use proconio::*;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let large: i64 = 1000000000000000000;
    let gcd = a.gcd(&b);
    let lcm = (a / gcd) * b;

    if a / gcd <= large / b {
        println!("{}", lcm);
    } else {
        println!("Large");
    }
}
