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

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

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

fn solve(h: usize, abs: Vec<[i32; 2]>) -> i32 {
    let mut dp = Vec::new();
    for _ in 0..(h + 1) {
        dp.push(-1);
    }

    dp[0] = 0;

    for k in 1..(h + 1) {
        let mut m = 10_i32.pow(9);
        for [a, b] in &abs {
            m = m.min(dp[(k as i32 - *a).max(0) as usize] + b)
        }

        dp[k] = m;
    }

    dp[h]
}

fn main() {
    input! {
        h: usize,
        n: usize,
        abs: [[i32; 2]; n],
    }

    println!(
        "{:?}",
        solve(h, abs.into_iter().map(|v| [v[0], v[1]]).collect())
    );
}
