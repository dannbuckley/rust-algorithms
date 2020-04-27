extern crate matrix;
extern crate priority_queue;

use matrix::prelude::*;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::vec::Vec;

// Kruskal's algorithm for constructing a minimum spanning tree
// Input: A weighted connected graph g = <V, E>
// Output: e_T, the set of edges composing a minimum spanning tree of g
fn kruskal(g: &Compressed<u8>) -> Vec<(usize, usize)> {
    // sort E in nondecreasing order of the edge weights
    // w(e_(i_1)) <= ... <= w(e_(i_|E|))
    let mut e = PriorityQueue::<(usize, usize), Reverse<u8>>::new();
    for i in 0..g.rows() {
        for j in (i + 1)..g.columns() {
            let w_ij = g.get((i, j));
            if w_ij > 0 {
                e.push((i, j), Reverse(w_ij));
            }
        }
    }
    let e_vec = e.into_sorted_vec();

    // initialize the set of tree edges and its size
    let mut e_t = Vec::<(usize, usize)>::new();
    let mut ecounter = 0;

    // keep track of vertices in tree to verify acyclicity
    let mut v_t = Vec::<usize>::new();

    // initialize the number of processed edges
    let mut k = 0;

    while ecounter < g.rows() - 1 {
        let e_ik = e_vec[k];
        k = k + 1;
        if !(v_t.contains(&e_ik.0) && v_t.contains(&e_ik.1)) {
            e_t.push(e_vec[k - 1]);
            ecounter = ecounter + 1;

            if ecounter == 1 {
                v_t.push(e_ik.0);
                v_t.push(e_ik.1);
            }
            else if !v_t.contains(&e_ik.0) {
                v_t.push(e_ik.0);
            }
            else {
                v_t.push(e_ik.1);
            }
        }
    }

    e_t
}

fn main() {
    // example graph from figure 9.5 in book
    let mut g = Compressed::<u8>::zero((6, 6));
    
    // a
    g.set((0, 1), 3); // a -> b: 3
    g.set((0, 4), 6); // a -> e: 6
    g.set((0, 5), 5); // a -> f: 5

    // b
    g.set((1, 0), 3); // b -> a: 3
    g.set((1, 2), 1); // b -> c: 1
    g.set((1, 5), 4); // b -> f: 4

    // c
    g.set((2, 1), 1); // c -> b: 1
    g.set((2, 3), 6); // c -> d: 6
    g.set((2, 5), 4); // c -> f: 4

    // d
    g.set((3, 2), 6); // d -> c: 6
    g.set((3, 4), 8); // d -> e: 8
    g.set((3, 5), 5); // d -> f: 5

    // e
    g.set((4, 0), 6); // e -> a: 6
    g.set((4, 3), 8); // e -> d: 8
    g.set((4, 5), 2); // e -> f: 2

    // f
    g.set((5, 0), 5); // f -> a: 5
    g.set((5, 1), 4); // f -> b: 4
    g.set((5, 2), 4); // f -> c: 4
    g.set((5, 3), 5); // f -> d: 5
    g.set((5, 4), 2); // f -> e: 2

    let g_t = kruskal(&g);

    println!("Edges in minimum spanning tree (Kruskal):");
    for edge in g_t {
        println!("\t{} -> {}", edge.0, edge.1);
    }
}
