use algo::*;

fn solve(chs: Vec<char>) -> i32 {
    let mut r = 0;
    let mut pn = 0;
    for i in 0..chs.len() {
        // 出していいならパーを出す
        if i - pn > pn {
            pn += 1;

            if chs[i] == 'g' {
                r += 1;
            }
        } else {
            if chs[i] == 'p' {
                r -= 1;
            }
        }
    }

    r
}

fn main() {
    input! {
        s: String
    }

    println!("{:?}", solve(s.chars().collect()));
}
