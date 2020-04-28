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

    // label vector for vertices in V and U
    let mut labels = Vec::<isize>::new();
    for _ in 0..(l_v + l_u) {
        labels.push(-1);
    }

    // template vector for free vertices in V
    let mut v_free_set = Vec::<usize>::new();
    for _ in 0..l_v {
        v_free_set.push(1);
    }   

    while !q.is_empty() {
        let w = q.pop_front().unwrap();
        let mut v = 0;

        // if w in V
        if w < l_v {   
            let mut u = 0;
            while u < l_u {
                // if u is adjacent to w
                if g.get((w, u)) != 0 {
                    // if u is free
                    let mut u_free = true;
                    for i in 0..m.len() {
                        let pm_i = m[i];
                        if pm_i.0 == (u + l_v) || pm_i.1 == (u + l_v) {
                            u_free = false;
                        }
                    }
                    if u_free {
                        // augment
                        m.push((w, u + l_v));
                        v = w;

                        // while v is labeled
                        while labels[v] != -1 {
                            u = labels[v] as usize; // u = vertex indicated by v's label
                            for r_i in 0..m.len() {
                                if m[r_i] == (v, u) {
                                    m.remove(r_i);
                                    break;
                                }
                            }
                            v = labels[u] as usize; // v = vertex indicated by u's label
                            m.push((v, u));
                        }

                        // remove all vertex labels
                        for i in 0..labels.len() {
                            labels[i] = -1;
                        }

                        // reinitialize q with all free vertices in V
                        q.clear();
                        let mut f_k = v_free_set.clone();
                        for p in 0..m.len() {
                            if m[p].0 < l_v {
                                f_k[m[p].0] = 0;
                            }
                            else if m[p].1 < l_v {
                                f_k[m[p].1] = 0;
                            }
                        }
                        for i in 0..l_v {
                            if f_k[i] == 1 {
                                q.push_back(i);
                            }
                        }          
                        break;
                    } else {
                        // u is matched
                        if !m.contains(&(w, u + l_v)) && labels[u] == -1 {
                            labels[u + l_v] = w as isize; // label u with w
                            q.push_back(u + l_v);
                        }
                    }
                }

                u = u + 1;
            }        
        } else {
            // w in U (and matched)
            // label the mate v of w with w
            for i in 0..m.len() {
                let pm_i = m[i];
                if pm_i.0 == w {
                    v = pm_i.1;
                    break;
                }
                else if pm_i.1 == w {
                    v = pm_i.0;
                    break;
                }
            }
            labels[v] = w as isize;
            q.push_back(v);
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

    println!("Original graph:");
    for i in 0..g.rows() {
        for j in 0..g.columns() {
            if g.get((i, j)) != 0 {
                println!("\t{} -> {}", i, j + g.rows());
            }
        }
    }

    let m = maximum_bipartite_matching(&g);

    println!("Edges in maximum matching:");
    for edge in m {
        println!("\t{} -> {}", edge.0, edge.1);
    }
}
