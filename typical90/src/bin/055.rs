use algo::*;

fn solve(n: usize, p: i64, q: i64, an: Vec<i64>) -> i64 {
    let mut c = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        if (((((((an[i] * an[j]) % p) * an[k]) % p) * an[l]) % p) * an[m]) % p == q
                        {
                            c += 1;
                        }
                    }
                }
            }
        }
    }

    c
}

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        an: [i64; n],
    }

    println!("{}", solve(n, p, q, an));
}
