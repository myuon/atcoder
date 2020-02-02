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

fn solve(_a: usize, _b: usize, _q: usize, s: Vec<i64>, t: Vec<i64>, xs: Vec<i64>) {
    let big = 10_i64.pow(13);
    for x in xs {
        let r = match (s.binary_search(&x), t.binary_search(&x)) {
            (Ok(_), Ok(_)) => 0,
            (Ok(_), Err(ti)) => {
                let (t1, t2) = (
                    if ti == 0 { -big } else { t[ti - 1] },
                    if ti == t.len() { big } else { t[ti] },
                );

                std::cmp::min(x - t1, t2 - x)
            }
            (Err(si), Ok(_)) => {
                let (s1, s2) = (
                    if si == 0 { -big } else { s[si - 1] },
                    if si == s.len() { big } else { s[si] },
                );

                std::cmp::min(x - s1, s2 - x)
            }
            (Err(si), Err(ti)) => {
                let (s1, s2) = (
                    if si == 0 { -big } else { s[si - 1] },
                    if si == s.len() { big } else { s[si] },
                );
                let (t1, t2) = (
                    if ti == 0 { -big } else { t[ti - 1] },
                    if ti == t.len() { big } else { t[ti] },
                );

                vec![
                    (x - s1) + (t2 - x) + std::cmp::min(x - s1, t2 - x),
                    (s2 - x) + (x - t1) + std::cmp::min(s2 - x, x - t1),
                    std::cmp::max(x - s1, x - t1),
                    std::cmp::max(s2 - x, t2 - x),
                ]
                .into_iter()
                .min()
                .unwrap()
            }
        };

        println!("{:?}", r);
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        s: [i64; a],
        t: [i64; b],
        xs: [i64; q],
    }

    solve(a, b, q, s, t, xs);
}
