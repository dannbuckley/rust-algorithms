extern crate matrix;

use matrix::prelude::*;
use std::vec::Vec;

// Implements the memory function method for the knapsack problem
// Input: A nonnegative integer i indicating the number of the first items
//     being considered and a nonnegative integer j indicating the knapsack capacity
// Output: The value of an optimal feasible subset of the first i items
fn mf_knapsack(f: &mut Compressed<f64>, w: &Vec<f64>, v: &Vec<f64>, i: usize, j: usize) -> f64 {
    if f.get((i, j)) < 0.0 {
        let value;
        if j < (w[i - 1] as usize) {
            value = mf_knapsack(f, w, v, i - 1, j);
        } else {
            let l = mf_knapsack(f, w, v, i - 1, j);
            let r = v[i - 1] + mf_knapsack(f, w, v, i - 1, j - (w[i - 1] as usize));
            if l > r {
                value = l;                
            } else {
                value = r;
            }
        }
        f.set((i, j), value);
    }
    f.get((i, j))
}

fn main() {
    // initialize table values with -1 except for row 0 and column 0
    let mut f = Compressed::<f64>::zero((5, 6));
    for i in 1..5 {
        for j in 1..6 {
            f.set((i, j), -1.0);
        }
    }

    // use example weights and values from figure 8.6 in book
    let w: Vec::<f64> = vec![2.0, 1.0, 3.0, 2.0];
    let v: Vec::<f64> = vec![12.0, 10.0, 20.0, 15.0];

    println!("Weights: {:?}", w);
    println!("Values: {:?}", v);
    println!("Capacity W = 5");

    let max_knapsack = mf_knapsack(&mut f, &w, &v, 4, 5);
    println!("Maximum value of knapsack: {}", max_knapsack);
}
