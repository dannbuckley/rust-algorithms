extern crate math;

use math::round;
use std::vec::Vec;

// Finds distance between two closest points in the plane by brute force
// Input: A list p of n (n >= 2) points p_1(x_1, y_1), ..., p_n(x_n, y_n)
// Output: The distance between the closest pair of points
fn brute_force_closest_pair(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    let mut d = f64::INFINITY;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let xd = p[i].0 - p[j].0;
            let yd = p[i].1 - p[j].1;
            let e = xd.powi(2) + yd.powi(2);
            d = d.min(e.sqrt());            
        }
    }
    d
}

// Solves the closest-pair problem by divide-and-conquer
// Input: An array p of n >= 2 points in the Cartesian plane sorted in
//     nondecreasing order of their x coordinates and an array q of the same
//     points sorted in nondecreasing order of the y coordinates
// Output: Euclidean distance between the closest pair of points
fn efficient_closest_pair(p: &[(f64, f64)], q: &[(f64, f64)]) -> f64 {
    let n = p.len();
    if n <= 3 {
        return brute_force_closest_pair(p);
    } else {
        let nch = round::ceil(n as f64 / 2.0, 0) as usize;
        let dl = efficient_closest_pair(&p[0..nch], &q[0..nch]);
        let dr = efficient_closest_pair(&p[nch..n], &q[nch..n]);
        let d: f64;
        if dl <= dr {
            d = dl;
        } else {
            d = dr;
        }

        let m = p[nch - 1].0;
        let mut s = Vec::<(f64, f64)>::new();
        for y_point in q {
            let diff: f64 = y_point.0 - m;
            if diff.abs() < d {
                s.push(*y_point);
            }
        }
        let num = s.len();

        let mut dminsq: f64 = d.powi(2);
        for i in 0..(num - 1) {
            let mut k = i + 1;
            while (k <= num - 1) && (s[k].1 - s[i].1).powi(2) < dminsq {
                let e_dist: f64 = (s[k].0 - s[i].0).powi(2) + (s[k].1 - s[i].1).powi(2);
                if e_dist < dminsq {
                    dminsq = e_dist;
                }
                k = k + 1;
            }
        }
        return dminsq.sqrt();
    }
}

fn main() {
    let p: [(f64, f64); 4] = [(0f64, 0f64),(1f64, 1f64), (2f64, 5f64), (7f64, 3f64)];
    let q: [(f64, f64); 4] = [(0f64, 0f64),(1f64, 1f64), (7f64, 3f64), (2f64, 5f64)];
    println!("Points: {:?}", p);    
    println!("Distance between closest pair of points: {}", efficient_closest_pair(&p, &q));
}
