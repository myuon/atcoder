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

fn solve(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> (i32, i32) {
    let mut result = (1, 0);

    for x in 0..=f {
        if 100 * a * x > f {
            break;
        }

        for y in 0..=f {
            if 100 * a * x + 100 * b * y > f {
                break;
            }

            for z in 0..=f {
                if 100 * a * x + 100 * b * y + c * z > f {
                    break;
                }
                if c * z > (a * x + b * y) * e {
                    break;
                }

                for w in 0..=f {
                    if 100 * a * x + 100 * b * y + c * z + d * w > f {
                        break;
                    }
                    if c * z + d * w > (a * x + b * y) * e {
                        break;
                    }

                    let new = (100 * a * x + 100 * b * y + c * z + d * w, c * z + d * w);
                    if new.1 * result.0 >= new.0 * result.1 {
                        result = new;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32
    }

    let (x, y) = solve(a, b, c, d, e, f);
    println!("{} {}", x, y);
}
