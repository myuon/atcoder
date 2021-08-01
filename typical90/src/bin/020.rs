use proconio::*;

fn main() {
    input! {
        a: i64,
        b: u32,
        c: i64,
    }

    println!("{}", if c.pow(b) > a { "Yes" } else { "No" });
}
