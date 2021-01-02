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

fn solve(n: usize, sn: &mut Vec<String>) -> i32 {
    let mut count = 0;

    for i in 0..n {
        let r = sn[i]
            .chars()
            .enumerate()
            .filter(|(_, k)| *k == '.')
            .map(|x| x.0)
            .max();

        if let Some(k) = r {
            count += 1;

            if i == n - 1 {
                break;
            }
            sn[i + 1] = sn[i + 1][0..k].to_string();
        }
    }

    count
}

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let mut sn_ = sn;
    println!("{}", solve(n, &mut sn_));
}
