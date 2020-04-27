extern crate matrix;

use matrix::prelude::*;
use std::cmp;

// Implements Floyd's algorithm for the all-pairs shortest-paths problem
// Input: The weight matrix w of a graph with no negative-length cycle
// Output: The distance matrix of the shortest paths' lengths
fn floyd(w: &mut Compressed<u8>) {
    let n = w.rows();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let d_ij = w.get((i, j));
                let d_ik_kj: u8;
                if w.get((i, k)) | w.get((k, j)) == 255 {
                    d_ik_kj = 255;
                } else {
                    d_ik_kj = w.get((i, k)) + w.get((k, j));
                }
                w.set((i, j), cmp::min::<u8>(d_ij, d_ik_kj));
            }
        }
    }    
}

fn main() {
    // example graph from figure 8.14 in book
    let mut w = Compressed::<u8>::zero((4, 4));

    w.set((0, 2), 3); // a -> c: 3
    w.set((1, 0), 2); // b -> a: 2
    w.set((2, 1), 7); // c -> b: 7
    w.set((2, 3), 1); // c -> d: 1
    w.set((3, 0), 6); // d -> a: 6

    // set unused weights to high value
    w.set((0, 1), 255);
    w.set((0, 3), 255);
    w.set((1, 2), 255);
    w.set((1, 3), 255);
    w.set((2, 0), 255);
    w.set((3, 1), 255);
    w.set((3, 2), 255);

    println!("Original graph:");
    for i in 0..4 {
        for j in 0..4 {
            if w.get((i, j)) > 0 && w.get((i, j)) != 0xff {
                println!("\t{} -> {}: {}", i, j, w.get((i, j)));
            }
        }
    }

    floyd(&mut w);

    println!("Distance matrix:");
    for i in 0..4 {
        println!("\t{}\t{}\t{}\t{}", w.get((i, 0)), w.get((i, 1)), w.get((i, 2)), w.get((i, 3)));
    }
}
