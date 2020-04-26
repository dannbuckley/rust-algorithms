extern crate matrix;

use matrix::prelude::*;
use std::vec::Vec;

// Applies Gaussian elimination to matrix a of a system's coefficients,
//     augmented with vector b of the system's right-hand side values
// Input: Matrix a[1..n, 1..n] and column-vector b[1..n]
// Output: An equivalent upper-triangular matrix in place of a with the
//     corresponding right-hand side values in the (n + 1)st column
fn forward_elimination(a: &Conventional<f64>, b: &Vec<f64>) -> Compressed<f64> {
    let a_r = a.rows();
    let a_c = a.columns();
    let n = b.len();
    let mut c = Compressed::<f64>::from(a);
    c.resize((a_r, a_c + 1));

    for i in 0..a_r {
        c.set((i, a_c), b[i]); // augments the matrix
    }

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            // compute scaling factor outside of k loop as it does not depend on k
            let scaling_factor = c.get((j, i)) / c.get((i, i));
            for k in i..(n + 1) {
                c.set((j, k), c.get((j, k)) - c.get((i, k)) * scaling_factor);
            }
        }
    }

    c
}

fn main() {
    // use example system from book
    let mut a = Compressed::<f64>::zero((3, 3));
    a.set((0, 0), 2.0);
    a.set((0, 1), -1.0);
    a.set((0, 2), 1.0);
    a.set((1, 0), 4.0);
    a.set((1, 1), 1.0);
    a.set((1, 2), -1.0);
    a.set((2, 0), 1.0);
    a.set((2, 1), 1.0);
    a.set((2, 2), 1.0);

    println!("Original matrix:");
    for i in 0..3 {
        println!("\t{:?}\t{:?}\t{:?}", a.get((i, 0)), a.get((i, 1)), a.get((i, 2)));
    }

    let con_a = Conventional::from(a);
    let b: Vec::<f64> = vec![1.0, 5.0, 0.0];
    println!("Column vector: {:?}", b);

    let c = forward_elimination(&con_a, &b);

    println!("Upper-triangular matrix after Gaussian elimination:");
    for i in 0..3 {
        println!("\t{:?}\t{:?}\t{:?}\t| {:?}", c.get((i, 0)), c.get((i, 1)), c.get((i, 2)), c.get((i, 3)))
    }
}
