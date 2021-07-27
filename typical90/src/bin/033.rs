use algo::*;

fn solve(h: i32, w: i32) -> i32 {
    if h == 1 || w == 1 {
        h.max(w)
    } else {
        ((h + 1) / 2) * ((w + 1) / 2)
    }
}

fn main() {
    input! {
        h: i32,
        w: i32
    }

    println!("{}", solve(h, w));
}
