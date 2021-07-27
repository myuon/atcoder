use algo::*;

fn solve(n: usize, k: usize, an: Vec<i32>, bn: Vec<i32>) -> bool {
    let mut diff = 0;
    for i in 0..n {
        diff += (an[i] - bn[i]).abs();
    }

    diff <= k as i32 && (k as i32 - diff).abs() % 2 == 0
}

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i32; n],
        bn: [i32; n],
    }

    println!("{}", if solve(n, k, an, bn) { "Yes" } else { "No" });
}
