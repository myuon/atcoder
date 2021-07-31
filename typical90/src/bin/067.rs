use algo::*;

fn parse_as_10(x: Vec<i32>) -> i64 {
    let mut n = 0;
    let mut k = 1;
    for i in (0..x.len()).rev() {
        n += x[i] as i64 * k;
        k *= 8;
    }

    n
}

fn unparse_as_9(x: i64) -> Vec<i32> {
    let mut x = x;
    let mut result_rev = vec![];

    while x > 0 {
        let r = x % 9;
        result_rev.push(r as i32);
        x /= 9;
    }

    result_rev.reverse();

    result_rev
}

fn solve(ns: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = ns;

    for _ in 0..k {
        result = unparse_as_9(parse_as_10(result))
            .into_iter()
            .map(|c| if c == 8 { 5 } else { c })
            .collect();
    }

    result
}

fn main() {
    input! {
        n: String,
        k: i32,
    }

    if n == "0" {
        println!("0")
    } else {
        println!(
            "{}",
            solve(
                n.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i32)
                    .collect(),
                k,
            )
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
        );
    }
}
