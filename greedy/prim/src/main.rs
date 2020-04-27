extern crate matrix;

use matrix::prelude::*;
use std::vec::Vec;

// Prim's algorithm for constructing a minimum spanning tree
// Input: A weighted connected graph g = <V, E>
// Output: e_T, the set of edges composing a minimum spanning tree of g
fn prim(g: &Compressed<u8>) -> Vec<(usize, usize)> {
    let mut v_t = Vec::<usize>::new();
    v_t.push(0); // start at first vertex
    let mut e_t = Vec::<(usize, usize)>::new();

    for _ in 1..g.rows() {
        let mut w: u8 = 0;
        let mut u: usize = 0;
        let mut v: usize = 0;

        // find a minimum-weight edge e^* = (v^*, u^*) among all the edges (v, u)
        // such that v is in v_t and u is in V - v_t
        for i in 0..v_t.len() {
            for j in 0..g.columns() {
                let w_ij = g.get((v_t[i], j));
                if !v_t.contains(&j) && w_ij > 0 {
                    if w == 0 || w_ij < w {
                        w = w_ij;
                        u = j;
                        v = v_t[i];
                    }
                }
            }
        }
        
        v_t.push(u);
        e_t.push((v, u));
    }

    e_t
}

fn main() {
    // example graph from figure 9.3 in book
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

    let g_t = prim(&g);

    println!("Edges in minimum spanning tree (Prim):");
    for edge in g_t {
        println!("\t{} -> {}", edge.0, edge.1);
    }
}
