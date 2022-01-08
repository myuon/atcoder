use algo::*;

fn solve(n: usize, aij: Vec<Vec<usize>>) -> i64 {
    let mut result = 1;
    for i in aij {
        let mut s = 0;
        for j in i {
            s += j;
        }

        result = (result * s as i64) % (1000000000 + 7);
    }

    result
}

fn main() {
    input! {
        n: usize,
        aij: [[usize; 6]; n],
    }

    println!("{}", solve(n, aij));
}
