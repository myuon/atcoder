use algo::*;

fn binary_search(predicate: impl Fn(usize) -> bool) -> usize {
    // search span: [left, right)
    let mut left = 0;
    let mut right = 1;
    while predicate(right) {
        right *= 2;
    }

    while right - left > 1 {
        let mid = (left + right) / 2;
        if predicate(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn solve(n: usize, l: usize, k: usize, mut an: Vec<i64>) {
    an.push(l as i64);

    let mut bn = vec![];
    let mut prev = 0;
    for a in an {
        bn.push(a - prev);
        prev = a;
    }

    let result = binary_search(|u| {
        let mut count = 0;
        let mut current = 0;
        for i in 0..=n {
            current += bn[i];
            if current >= u as i64 {
                current = 0;
                count += 1;
            }
        }

        count > k
    });

    println!("{}", result);
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        an: [i64; n],
    }

    solve(n, l, k, an);
}
