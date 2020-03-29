use algo::*;
use std::collections::HashMap;

fn solve(s: Vec<char>, t: Vec<char>) -> i64 {
    let mut s_count = HashMap::new();
    for i in 0..s.len() {
        s_count.entry(s[i]).or_insert(Vec::new()).push(i);
    }

    let mut i = 0;
    let mut r = 0;
    for ch in t {
        if !s_count.contains_key(&ch) {
            return -1;
        }

        let may_found = s_count[&ch].binary_search(&(i as usize));
        match may_found {
            Ok(ui) | Err(ui) if ui < s_count[&ch].len() => {
                let u = s_count[&ch][ui];
                r += u as i64 - i as i64 + 1;
                i = u as i64 + 1;
            }
            _ => {
                let h = s_count[&ch][0];
                r += s.len() as i64 - i as i64 + h as i64 + 1;
                i = h as i64 + 1;
            }
        }
    }

    r
}

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{:?}", solve(s.chars().collect(), t.chars().collect()));
}
