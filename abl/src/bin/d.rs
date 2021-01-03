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

pub struct MaxSegmentTree<T> {
    data: Vec<T>,
    size: usize,
    default: T,
}

impl<T: Copy + Ord> MaxSegmentTree<T> {
    pub fn new(data: Vec<T>, default: T) -> Self {
        let mut w = data.len();
        let mut size = 2;
        while w > 0 {
            size *= 2;
            w = w / 2;
        }

        let mut t = MaxSegmentTree {
            data: vec![default; size],
            size: size / 2,
            default,
        };
        for (i, val) in data.iter().enumerate() {
            t.update(i, *val);
        }

        t
    }

    pub fn update(&mut self, index: usize, val: T) {
        let mut prev = val;
        let mut current = index + self.size - 1;
        self.data[current] = val;

        loop {
            if current == 0 {
                break;
            }

            current = (current - 1) / 2;

            self.data[current] = self.data[current].max(prev);
            prev = self.data[current];
        }
    }

    pub fn query(&self, s: usize, t: usize) -> T {
        let mut check = vec![(s, t, 0, 0, self.size)];
        let mut answer = self.default;

        while let Some((a, b, k, l, r)) = check.pop() {
            if r <= a || b <= l {
                answer = answer.max(self.default);
                continue;
            }

            if a <= l && r <= b {
                answer = answer.max(self.data[k]);
                continue;
            }

            check.push((a, b, 2 * k + 1, l, (l + r) / 2));
            check.push((a, b, 2 * k + 2, (l + r) / 2, r));
        }

        answer
    }
}

const L: usize = 510_000;

fn solve(n: usize, k: usize, an: Vec<i32>) -> i32 {
    let mut t = MaxSegmentTree::new(vec![0; L], 0);
    for i in 0..n {
        if i == 0 {
            t.update(an[0] as usize, 1);
            continue;
        }

        t.update(
            an[i] as usize,
            t.query(
                (an[i] - k as i32).max(0) as usize,
                (an[i] + k as i32 + 1).min(L as i32) as usize,
            ) + 1,
        )
    }

    t.query(0, L)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i32; n],
    }

    println!("{}", solve(n, k, an));
}
