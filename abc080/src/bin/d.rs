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

fn solve(n: usize, _c: i32, stc: Vec<[i32; 3]>) -> i32 {
    let mut r = 0;

    let mut b = Vec::new();
    for _ in 0..n {
        b.push(false);
    }

    loop {
        let mut remains = Vec::new();
        for i in 0..n {
            if !b[i] {
                remains.push(i);
            }
        }

        if remains.len() == 0 {
            break;
        }

        remains.sort_by(|i, j| stc[*i][1].cmp(&stc[*j][1]));

        let mut prev_t = 0;
        let mut prev_c = 0;
        for r in remains {
            if stc[r][2] == prev_c {
                if prev_t <= stc[r][0] {
                    b[r] = true;
                    prev_t = stc[r][1];
                    prev_c = stc[r][2];
                }
            } else {
                if prev_t < stc[r][0] {
                    b[r] = true;
                    prev_t = stc[r][1];
                    prev_c = stc[r][2];
                }
            }
        }

        r += 1;
    }

    r
}

fn main() {
    input! {
        n: usize,
        c: i32,
        stc: [[i32; 3]; n]
    }

    println!(
        "{:?}",
        solve(n, c, stc.into_iter().map(|v| [v[0], v[1], v[2]]).collect())
    );
}
