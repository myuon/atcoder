#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn solve(n: usize, an: Vec<usize>) -> f64 {
    let mut count = vec![0, 0, 0];
    for a in an {
        count[a - 1] += 1;
    }

    let mut dp = vec![vec![vec![0.0; n + 1]; n + 1]; n + 1];

    for k in 0..=count[2] {
        for j in 0..=(count[1] + count[2]) {
            for i in 0..=n {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                if n < i + j + k {
                    continue;
                }
                let z = n - i - j - k;

                if i > 0 {
                    dp[i][j][k] += (i as f64 / (n - z) as f64) * dp[i - 1][j][k];
                }
                if j > 0 {
                    dp[i][j][k] += (j as f64 / (n - z) as f64) * dp[i + 1][j - 1][k];
                }
                if k > 0 {
                    dp[i][j][k] += (k as f64 / (n - z) as f64) * dp[i][j + 1][k - 1];
                }

                dp[i][j][k] += n as f64 / (n - z) as f64;
            }
        }
    }

    dp[count[0]][count[1]][count[2]]
}

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    println!("{}", solve(n, an));
}
