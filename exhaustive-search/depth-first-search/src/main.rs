extern crate matrix;

use matrix::prelude::*;
use std::vec::Vec;

// Implements a depth-first search traversal of a given graph
// Input: Graph g = <v, e>
// Output: Graph g with its vertices marked with consecution integers in the
//     order they are first encounter by the DFS traversal
fn depth_first_search(g: &Compressed<u8>) -> Compressed<u8> {
    let mut c = Compressed::<u8>::zero((g.rows(), g.columns()));
    let mut count = 0;
    let mut backcount = 0;
    let mut vertices = vec![(0, 0); g.rows()];

    for i in 0..g.rows() {
        if vertices[i].0 == 0 {
            depth_first_search_vertex(&g, &mut c, &mut vertices, i, &mut count, &mut backcount);
        };
    }

    c
}

// Visits recursively all the unvisited vertices connected to vertex v
//     by a path and numbers them in the order they are encountered
//     via global variable count
fn depth_first_search_vertex(g: &Compressed<u8>, g_f: &mut Compressed<u8>, v_set: &mut Vec<(u8, u8)>, v: usize, count: &mut u8, backcount: &mut u8) {
    *count  = *count + 1;
    v_set[v] = (*count, v_set[v].1);
    for i in 0..g.columns() {
        if g.get((v, i)) == 1 && v_set[i].0 == 0 {
            g_f.set((v, i), 1);
            depth_first_search_vertex(g, g_f, v_set, i, count, backcount);
        }
    }
    *backcount = *backcount + 1;
    v_set[v] = (v_set[v].0, *backcount);
}

fn main() {
    // use example graph from book (Figure 3.10)
    // represent graph with adjacency matrix
    let mut graph = Compressed::<u8>::zero((10, 10));
    // a
    graph.set((0, 2), 1);
    graph.set((0, 3), 1);
    graph.set((0, 4), 1);

    // b
    graph.set((1, 4), 1);
    graph.set((1, 5), 1);

    // c
    graph.set((2, 0), 1);
    graph.set((2, 3), 1);
    graph.set((2, 5), 1);

    // d
    graph.set((3, 0), 1);
    graph.set((3, 2), 1);

    // e
    graph.set((4, 0), 1);
    graph.set((4, 1), 1);
    graph.set((4, 5), 1);

    // f
    graph.set((5, 1), 1);
    graph.set((5, 2), 1);
    graph.set((5, 4), 1);

    // g
    graph.set((6, 7), 1);
    graph.set((6, 9), 1);

    // h
    graph.set((7, 6), 1);
    graph.set((7, 8), 1);

    // i
    graph.set((8, 7), 1);
    graph.set((8, 9), 1);

    // j
    graph.set((9, 6), 1);
    graph.set((9, 8), 1);

    let final_graph = depth_first_search(&graph);

    println!("Edges in DFS Forest (3.10):");
    for x in 0..10 {
        for y in 0..10 {
            if final_graph.get((x, y)) == 1 {
                println!("\t {} to {}", x, y);
            }
        }
    }

    // example graph from Figure 4.5 in section on "Topological Sorting"
    let mut digraph = Compressed::<u8>::zero((5, 5));

    //a
    digraph.set((0, 1), 1);
    digraph.set((0, 2), 1);

    //b
    digraph.set((1, 0), 1);
    digraph.set((1, 2), 1);

    //d
    digraph.set((3, 2), 1);
    digraph.set((3, 4), 1);

    let final_digraph = depth_first_search(&digraph);

    println!("Edges in DFS Forest (4.5):");
    for x in 0..5 {
        for y in 0..5 {
            if final_digraph.get((x, y)) == 1 {
                println!("\t {} to {}", x, y);
            }
        }
    }
}
