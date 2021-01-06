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

const BIG: i64 = 1000000007;

fn solve(n: usize, cn: Vec<usize>) -> i64 {
    let mut prev = vec![-1; n];
    let mut count = vec![-1; 1000000];
    for (i, c) in cn.into_iter().enumerate() {
        if count[c] != -1 {
            prev[i] = count[c] as i64;
        }

        count[c] = i as i64;
    }

    let mut dp = vec![0; n];
    for i in 0..n {
        if i == 0 {
            dp[0] = 1;
            continue;
        }

        dp[i] = dp[i - 1];

        let p = prev[i];
        if p >= 0 && (p as usize) < i - 1 {
            dp[i] = (dp[i] + dp[p as usize]) % BIG;
        }
    }

    dp[n - 1]
}

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    println!("{}", solve(n, c));
}
