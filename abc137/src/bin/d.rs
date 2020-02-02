use std::collections::BinaryHeap;

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

fn solve(_n: i32, m: i32, abs: Vec<[i32; 2]>) -> i32 {
    let mut plans = Vec::new();
    for _ in 0..(m + 1) {
        plans.push(Vec::new());
    }

    for [a, b] in abs {
        if m >= a {
            plans[a as usize].push(b);
        }
    }

    let mut doable = BinaryHeap::new();
    let mut r = 0;

    for d in 0..(m + 1) {
        for p in &plans[d as usize] {
            doable.push(p);
        }

        if let Some(k) = doable.pop() {
            r += k;
        }
    }

    r
}

fn main() {
    input! {
        n: i32,
        m: i32,
        abs: [[i32; 2]; n],
    }

    println!(
        "{:?}",
        solve(n, m, abs.into_iter().map(|v| [v[0], v[1]]).collect())
    );
}
