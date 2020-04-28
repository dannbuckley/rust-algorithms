extern crate matrix;
extern crate priority_queue;

use matrix::prelude::*;
use priority_queue::PriorityQueue;
use std::vec::Vec;

// Dijkstra's algorithm for single-source shortest paths
// Input: A weighted connected graph g = <V, E> with nonnegative weights and its vertex s
// Output: The length d_v of a shortest path from s to v and its penultimate vertex p_v
//     for every vertex v in V
fn dijkstra(g: &Compressed<u8>, s: usize) -> (Vec<u8>, Vec<isize>) {
    // initialize priority queue to empty
    let mut q = PriorityQueue::<usize, u8>::new();

    // initialize vectors for penultimate vertices and shortest distances
    let mut p = Vec::<isize>::new();
    let mut d = Vec::<u8>::new();

    for v in 0..g.rows() {
        d.push(255);
        p.push(-1);
        q.push(v, 255 - d[v]); // initialize vertex priority in the priority queue
    }

    d[s] = 0;
    q.push_increase(s, 255 - d[s]); // update priority of s with d_s
    let mut v_t = Vec::<usize>::new();

    for _ in 0..g.rows() {
        // delete the minimum priority element
        let min = q.pop().unwrap();
        let u_star = min.0;
        v_t.push(u_star);

        for u in 0..g.columns() {
            // for every vertex u in V - v_t that is adjacent to u_star
            if !v_t.contains(&u) && g.get((u_star, u)) != 0 {
                if d[u_star] + g.get((u_star, u)) < d[u] {
                    d[u] = d[u_star] + g.get((u_star, u));
                    p[u] = u_star as isize;
                    q.push_increase(u, 255 - d[u]);
                }
            }
        }
    }

    (d, p)
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

    let djk = dijkstra(&g, 0);
    let d = djk.0;
    let p = djk.1;

    for i in 1..d.len() {
        println!("Source (0) -> {}:", i);
        println!("\tShortest distance: {}", d[i]);
        println!("\tPenultimate vertex: {}", p[i]);
    }
}
