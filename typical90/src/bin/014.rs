use proconio::*;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        mut bn: [i64; n],
    }

    an.sort();
    bn.sort();

    let mut result = 0;
    for i in 0..n {
        result += (an[i] - bn[i]).abs();
    }

    println!("{}", result);
}
