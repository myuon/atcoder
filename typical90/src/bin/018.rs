use std::f64::consts::PI;

use proconio::*;

fn main() {
    input! {
        t: i64,
        l: i64,
        x: i64,
        y: i64,
        q: usize,
        es: [i64; q]
    }

    for e in es {
        let theta = 2.0 * PI * (e as f64) / (t as f64);
        let u = (l as f64 / 2.0) * (1.0 - theta.cos());
        let v = (x.pow(2) as f64 + (y as f64 - (l as f64 / 2.0) * (-theta.sin())).powi(2)).sqrt();
        println!("{}", u.atan2(v).to_degrees());
    }
}
