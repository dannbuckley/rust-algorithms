extern crate matrix;
extern crate priority_queue;

use matrix::prelude::*;
use priority_queue::PriorityQueue;
use std::vec::Vec;

// Dijkstra's algorithm for single-source shortest paths
// Input: A weighted connected graph g = <V, E> with nonnegative weights and its vertex s
// Output: The length d_v of a shortest path from s to v and its penultimate vertex p_v
//     for every vertex v in V
fn dijkstra(g: &Compressed<u8>, s: usize) {
    // TODO
}

fn main() {
    // example graph from figure 9.11 in book
    let mut g = Compressed::<u8>::zero((5, 5));

    // a
    g.set((0, 1), 3); // a -> b: 3
    g.set((0, 3), 7); // a -> d: 7

    // b
    g.set((1, 0), 3); // b -> a: 3
    g.set((1, 2), 4); // b -> c: 4
    g.set((1, 3), 2); // b -> d: 2

    // c
    g.set((2, 1), 4); // c -> b: 4
    g.set((2, 3), 5); // c -> d: 5
    g.set((2, 4), 6); // c -> e: 6

    // d
    g.set((3, 0), 7); // d -> a: 7
    g.set((3, 1), 2); // d -> b: 2
    g.set((3, 2), 5); // d -> c: 5
    g.set((3, 4), 4); // d -> e: 4

    // e
    g.set((4, 2), 6); // e -> c: 6
    g.set((4, 3), 4); // e -> d: 4

    dijkstra(&g, 0);

    println!("Hello, world!");
}
