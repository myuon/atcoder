use algo::*;

fn solve(n: usize, l: usize) -> i64 {
    let mut dp = vec![0; n + 1];

    dp[0] = 1;
    for i in 1..=n {
        dp[i] = (if i >= 1 { dp[i - 1] } else { 0 } + if i >= l { dp[i - l] } else { 0 })
            % (1000000000 + 7);
    }

    dp[n]
}

fn main() {
    input! {
        n: usize,
        l: usize
    }

    println!("{}", solve(n, l));
}
