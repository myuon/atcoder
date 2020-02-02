macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
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

fn solve(a: i64, b: i64) -> i64 {
    let mut fs = Vec::new();
    for i in 1..=((a as f64).sqrt().floor() as i64) {
        if a % i == 0 {
            fs.push(i);
        }
    }

    let mut gs = Vec::new();
    for f in fs {
        if b % f == 0 {
            gs.push(f);
        }
    }

    let mut c = 0;
    for i in 0..gs.len() {
        let g1 = gs[i];
        if g1 != 0 {
            c += 1;
        }
        if g1 == 1 {
            continue;
        }

        for j in i..gs.len() {
            let g2 = gs[j];
            if g1 != 0 && g2 % g1 == 0 {
                gs[j] = 0;
            }
        }
    }

    c
}

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!("{:?}", solve(a, b));
}
