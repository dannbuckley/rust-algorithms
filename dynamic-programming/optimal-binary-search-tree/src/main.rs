extern crate matrix;

use matrix::prelude::*;
use std::vec::Vec;

// Finds an optimal binary search tree by dynamic programming
// Input: An array p[1..n] of search probabilities for a sorted list of n keys
// Output: Average number of comparisons in successful searches in the
//     optimal BST and table r of subtrees' roots in the optimal BST
fn optimal_bst(p: &Vec<f64>) -> (f64, Compressed<usize>) {
    let n = p.len();
    let mut c = Compressed::<f64>::zero((n + 2, n + 1));
    let mut r = Compressed::<usize>::zero((n + 2, n + 1));

    for i in 1..(n + 1) {
        c.set((i, i - 1), 0.0);
        c.set((i, i), p[i - 1]);
        r.set((i, i), i);
    }
    c.set((n + 1, n), 0.0);

    // diagonal count
    for d in 1..n {
        for i in 1..(n - d + 1) {
            let j = i + d;
            let mut minval = f64::INFINITY;
            let mut kmin = 0;
            
            for k in i..(j + 1) {
                let c_sum = c.get((i, k - 1)) + c.get((k + 1, j));
                if c_sum < minval {
                    minval = c_sum;
                    kmin = k;
                }
            }
            r.set((i, j), kmin);
            
            let mut sum = p[i - 1];
            for s in (i + 1)..(j + 1) {
                sum = sum + p[s - 1];
            }
            c.set((i, j), minval + sum);
        }
    }

    (c.get((1, n)), r)
}

fn main() {
    // use example search probabilities from book
    let p: Vec::<f64> = vec![0.1, 0.2, 0.4, 0.3];
    let opt = optimal_bst(&p);
    println!("Average number of comparisons in successful searches: {}", opt.0);
    
    println!("Root table:");
    for i in 1..opt.1.rows() {
        for j in 0..opt.1.columns() {
            print!("\t{}", opt.1.get((i, j)));
        }
        print!("\n");
    }
}
