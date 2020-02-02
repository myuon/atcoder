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

fn solve(n: usize, _m: usize, _q: usize, lrs: Vec<[i32; 2]>, pqs: Vec<[i32; 2]>) {
    let mut graph = [[0; 500]; 500];
    for [l, r] in lrs {
        graph[l as usize - 1][r as usize - 1] += 1;
    }

    let mut accum = [[0; 510]; 510];
    for i in 0..n {
        let mut d = 0;
        for j in 0..n {
            d += graph[i][j];
            accum[i + 1][j + 1] = d + accum[i][j + 1];
        }
    }

    for [p, q] in pqs {
        println!(
            "{:?}",
            accum[q as usize][q as usize]
                - accum[p as usize - 1][q as usize]
                - accum[q as usize][p as usize - 1]
                + accum[p as usize - 1][p as usize - 1]
        );
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lrs: [[i32; 2]; m],
        pqs: [[i32; 2]; q],
    }

    solve(
        n,
        m,
        q,
        lrs.into_iter().map(|v| [v[0], v[1]]).collect(),
        pqs.into_iter().map(|v| [v[0], v[1]]).collect(),
    );
}
