use std::collections::VecDeque;

use algo::*;

fn solve(q: usize, txn: Vec<(i32, i64)>) -> Vec<i64> {
    let mut acc = Vec::new();
    let mut deck = VecDeque::new();

    for (t, x) in txn {
        if t == 1 {
            deck.push_front(x);
        } else if t == 2 {
            deck.push_back(x);
        } else if t == 3 {
            acc.push(deck[x as usize - 1]);
        }
    }

    acc
}

fn main() {
    input! {
        q: usize,
        txn: [(i32, i64); q]
    }

    for v in solve(q, txn) {
        println!("{}", v);
    }
}
