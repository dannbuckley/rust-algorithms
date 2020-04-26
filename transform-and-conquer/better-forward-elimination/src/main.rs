extern crate matrix;

use matrix::prelude::*;

// Implements Gaussian elimination with partial pivoting
// Input: Matrix a[1..n, 1..n] and column-vector b[1..n]
// Output: An equivalent upper-triangular matrix in place of a and the
//     corresponding right-hand side value in place of the (n + 1)st column
fn better_forward_elimination(a: &Conventional<f64>, b: &Vec<f64>) -> Compressed<f64> {
    let a_r = a.rows();
    let a_c = a.columns();
    let n = b.len();
    let mut c = Compressed::<f64>::from(a);
    c.resize((a_r, a_c + 1));

    for i in 0..a_r {
        c.set((i, a_c), b[i]); // appends b to a as the last column
    }

    for i in 0..(n - 1) {
        let mut pivotrow = i;
        for j in (i + 1)..n {
            if c.get((j, i)).abs() > c.get((pivotrow, i)).abs() {
                pivotrow = j;
            }
        }
        for k in i..(n + 1) {
            let cik = c.get((i, k));
            c.set((i, k), c.get((pivotrow, k)));
            c.set((pivotrow, k), cik);
        }
        for j in (i + 1)..n {
            let temp = c.get((j, i)) / c.get((i, i));
            for k in i..(n + 1) {
                c.set((j, k), c.get((j, k)) - c.get((i, k)) * temp);
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

    let c = better_forward_elimination(&con_a, &b);

    println!("Upper-triangular matrix after Gaussian elimination (with partial pivoting):");
    for i in 0..3 {
        println!("\t{:?}\t{:?}\t{:?}\t| {:?}", c.get((i, 0)), c.get((i, 1)), c.get((i, 2)), c.get((i, 3)))
    }
}
