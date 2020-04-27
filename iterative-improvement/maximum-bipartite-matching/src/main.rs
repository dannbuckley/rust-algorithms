extern crate matrix;

use matrix::prelude::*;
use std::collections::VecDeque;
use std::vec::Vec;

// Finds a maximum matching in a bipartite graph by a BFS-like traversal
// Input: A bipartite graph g = <V, U, E>
// Output: A maximum-cardinality matching m in the graph
fn maximum_bipartite_matching(g: &Compressed<u8>) -> Vec<(usize, usize)> {
    let l_v = g.rows();
    let l_u = g.columns();

    // initialize set m of edges with some valid matching (e.g., the empty set)
    let mut m = Vec::<(usize, usize)>::new();

    // initialize queue q with all the free vertices in V (in any order)
    let mut q = VecDeque::<usize>::new();
    for v in 0..l_v {
        q.push_back(v);
    }

    while !q.is_empty() {
        let w = q.pop_front().unwrap();

        // if w in V
        if w < l_v {    
            for u in 0..l_u {
                // if u is free
            }        
        } else {
            // w in U (and matched)
        }
    }

    m
}

fn main() {
    // use example bipartite graph from figure 10.10 in book
    let mut g = Compressed::<u8>::zero((5, 5));

    // 1
    g.set((0, 0), 1); // V: 1 -> U: 6
    g.set((0, 1), 1); // V: 1 -> U: 7

    // 2
    g.set((1, 0), 1); // V: 2 -> U: 6

    // 3
    g.set((2, 0), 1); // V: 3 -> U: 6
    g.set((2, 2), 1); // V: 3 -> U: 8

    // 4
    g.set((3, 2), 1); // V: 4 -> U: 8
    g.set((3, 3), 1); // V: 4 -> U: 9
    g.set((3, 4), 1); // V: 4 -> U: 10

    // 5
    g.set((4, 3), 1); // V: 5 -> U: 9
    g.set((4, 4), 1); // V: 5 -> U: 10

    println!("Hello, world!");
}
