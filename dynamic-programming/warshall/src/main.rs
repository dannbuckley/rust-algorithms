extern crate matrix;

use matrix::prelude::*;

// Implements Warshall's algorithm for computing the transitive closure
// Input: The adjacency matrix a of a digraph with n vertices
// Output: The transitive closure of the digraph
fn warshall(a: &Conventional<u8>) -> Compressed<u8> {
    let mut r = Compressed::<u8>::from(a);
    let n = r.rows();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let r_ij = r.get((i, j));
                let r_ik_kj = r.get((i, k)) & r.get((k, j));
                r.set((i, j), r_ij | r_ik_kj);
            }
        }
    }

    r
}

fn main() {
    // example graph from figure 8.13 in book
    let mut a = Compressed::<u8>::zero((4, 4));

    a.set((0, 1), 1); // a -> b    
    a.set((1, 3), 1); // b -> d
    a.set((3, 0), 1); // d -> a
    a.set((3, 2), 1); // d -> c

    println!("Original graph:");
    for i in 0..a.rows() {
        for j in 0..a.columns() {
            if a.get((i, j)) != 0 {
                println!("\t{} -> {}", i, j);
            }
        }
    }

    let a_c = Conventional::<u8>::from(a);

    let a_r = warshall(&a_c);
    println!("Transitive closure:");
    for i in 0..a_r.rows() {
        for j in 0..a_r.columns() {
            if a_r.get((i, j)) != 0 {
                println!("\t{} -> {}", i, j);
            }
        }
    }
}
