use algo::*;

fn combs(n, k usize, m usize) -> Vec<i32> {
}

fn solve(k: usize, mut an: Vec<i32>) -> i32 {
    an.sort();
}

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i32; n],
    }

    println!("{:?}", solve(k, an));
}
