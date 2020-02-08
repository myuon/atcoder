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

fn solve(s: String) -> (i32, i32) {
    let chs: Vec<char> = s.chars().collect::<Vec<_>>();

    for ch in (b'a'..=b'z').map(char::from).collect::<Vec<_>>() {
        let mut prev = '0';
        let mut prev2 = '1';

        for i in 0..chs.len() {
            if chs[i] == prev {
                return (i as i32, (i + 1) as i32);
            }
            if chs[i] == prev2 {
                return ((i - 1) as i32, (i + 1) as i32);
            }

            prev2 = prev;
            prev = chs[i];
        }
    }

    (-1, -1)
}

fn main() {
    input! {
        s: String,
    }

    let (x, y) = solve(s);
    println!("{} {}", x, y);
}
