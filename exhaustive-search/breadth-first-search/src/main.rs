extern crate matrix;

use matrix::prelude::*;
use std::collections::VecDeque;
use std::vec::Vec;

// Implements a breadth-first search traversal of a given graph
// Input: Graph g = <v, e>
// Output: Graph g with its vertices marked with consecutive integers in the
//     order they are visited by the BFS traversal
fn breadth_first_search(g: &Compressed<u8>) -> Compressed<u8> {
    let mut c = Compressed::<u8>::zero((g.rows(), g.columns()));
    let mut count = 0;
    let mut vertices = vec![0; g.rows()];

    for i in 0..g.rows() {
        if vertices[i] == 0 {
            breadth_first_search_vertex(g, &mut c, &mut vertices, i, &mut count);
        }
    }

    c
}

// Visits all the unvisited vertices connected to vertex v by a path and
//     numbers them in the order they are visited via global variable count
fn breadth_first_search_vertex(g: &Compressed<u8>, g_f: &mut Compressed<u8>, v_set: &mut Vec<u8>, v: usize, count: &mut u8) {
    *count = *count + 1;
    v_set[v] = *count;
    let mut q = VecDeque::<u8>::new();
    q.push_back(v as u8);
    while !q.is_empty() {
        let front = *q.front().unwrap() as usize;
        for i in 0..g.columns() {
            if g.get((front, i)) == 1 && v_set[i] == 0 {
                g_f.set((front, i), 1);
                *count = *count + 1;
                v_set[i] = *count;
                q.push_back(i as u8);
            }
        }
        q.pop_front();
    }
}

fn main() {
    // use example graph from book
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
    
    let final_graph = breadth_first_search(&graph);

    println!("Edges in BFS Forest:");
    for x in 0..10 {
        for y in 0..10 {
            if final_graph.get((x, y)) == 1 {
                println!("\t {} to {}", x, y);
            }
        }
    }
}
