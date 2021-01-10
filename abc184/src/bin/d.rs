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

fn solve(a0: usize, b0: usize, c0: usize) -> f64 {
    let mut dp = vec![vec![vec![0.0; 200]; 200]; 200];
    dp[a0][b0][c0] = 1.0;

    for a in a0..100 {
        for b in b0..100 {
            for c in c0..100 {
                let s = a + b + c;

                if a > 0 {
                    dp[a + 1][b][c] += dp[a][b][c] * (a as f64 / s as f64);
                }
                if b > 0 {
                    dp[a][b + 1][c] += dp[a][b][c] * (b as f64 / s as f64);
                }
                if c > 0 {
                    dp[a][b][c + 1] += dp[a][b][c] * (c as f64 / s as f64);
                }
            }
        }
    }

    let mut result = 0.0;
    for a in a0..101 {
        for b in b0..101 {
            for c in c0..101 {
                if a == 100 || b == 100 || c == 100 {
                    result += dp[a][b][c] * (a - a0 + b - b0 + c - c0) as f64;
                }
            }
        }
    }

    result
}

fn main() {
    input! {
        a: usize,
        b:usize,
        c: usize,
    }

    println!("{}", solve(a, b, c));
}
