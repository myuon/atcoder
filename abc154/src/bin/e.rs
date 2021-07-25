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

fn comb(i: usize, j: usize) -> usize {
    match j {
        0 => 1,
        1 => i,
        2 => i * (i - 1) / 2,
        3 => i * (i - 1) * (i - 2) / 6,
        _ => todo!(),
    }
}

fn solve(n: Vec<char>, k: usize) -> i32 {
    let mut dp = vec![vec![vec![0; 2]; k + 1]; n.len()];

    for i in 0..n.len() {
        for j in 0..=k {
            if j == 0 {
                dp[i][j][0] = 1;
                dp[i][j][1] = 1;
                continue;
            }
            if i == 0 {
                dp[i][j][0] = 0;
                dp[i][j][1] = 0;
                continue;
            }

            let ni = n[i].to_string().parse::<i32>().unwrap();
            if ni == 0 {
                dp[i][j][0] = dp[i - 1][j][0]
            } else {
                dp[i][j][0] =
                    dp[i - 1][j][1] + dp[i - 1][j - 1][0] + (ni - 1) * dp[i - 1][j - 1][1];
            }
            dp[i][j][1] = comb(i, j) as i32 * 9i32.pow(j as u32);
        }
    }

    println!("{:?}", dp);

    dp[n.len() - 1][k][0] + dp[n.len() - 1][k][1]
}

fn main() {
    input! {
        n: String,
        k: usize,
    }

    println!("{}", solve(n.chars().collect(), k));
}
