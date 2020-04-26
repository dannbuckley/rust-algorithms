extern crate matrix;

use matrix::prelude::*;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::vec::Vec;

// Implements the shortest-augmenting-path algorithm
// Input: A network with single source 1, single sink n, and positive integer
//     capacities u_ij on its edges (i, j)
// Output: A maximum flow x
fn shortest_augmenting_path(u: &Compressed<u8>) -> Compressed<u8> {
    // assign x_ij = 0 to every edge (i, j) in the network
    let n = u.rows();
    let mut x = Compressed::<u8>::zero((u.rows(), u.columns()));
    let mut labels: Vec::<(f64, usize, bool)> = vec![(0.0, 0, false); n];
    
    // label the source with (INFINITY, 0)
    labels[0] = (f64::INFINITY, 0, false);

    // add the source to the empty queue q
    let mut q = VecDeque::<usize>::new();
    q.push_back(0);

    while !q.is_empty() {
        let mut i = q.pop_front().unwrap();

        // forward edges
        for j in 0..n {
            if u.get((i, j)) > 0 && labels[j].0 == 0.0 {
                if u.get((i, j)) > x.get((i, j)) {
                    let r_ij = u.get((i, j)) - x.get((i, j));
                    if labels[i].0 == f64::INFINITY {
                        labels[j].0 = r_ij as f64;
                    } else {
                        labels[j].0 = match r_ij.cmp(&(labels[i].0 as u8)) {
                            Ordering::Equal => r_ij as f64,
                            Ordering::Less => r_ij as f64,
                            Ordering::Greater => labels[i].0
                        };
                    }
                    labels[j].1 = i;
                    labels[j].2 = true;
                    q.push_back(j);
                }
            }
        }

        // backward edges
        for j in 0..n {
            if u.get((j, i)) > 0 && labels[j].0 == 0.0 {
                let x_ji = x.get((j, i));
                if x.get((j, i)) > 0 {
                    if labels[i].0 == f64::INFINITY {
                        labels[j].0 = x_ji as f64;
                    } else {
                        labels[j].0 = match x_ji.cmp(&(labels[i].0 as u8)) {
                            Ordering::Equal => x_ji as f64,
                            Ordering::Less => x_ji as f64,
                            Ordering::Greater => labels[i].0
                        };
                    }
                    labels[j].1 = i;
                    labels[j].2 = false;
                    q.push_back(j);
                }
            }
        }

        if labels[n -1].0 != 0.0 {
            // augment along the augmenting path found
            let mut j = n - 1; // start at the sink and move backwards using second labels
            while j != 0 {
                if labels[j].2 {
                    // the second label of vertex j is i^+
                    x.set((i, j), x.get((labels[j].1, j)) + labels[n - 1].0 as u8);

                } else {
                    // the second label of vertex j is i^-
                    x.set((j, i), x.get((j, labels[j].1)) - labels[n - 1].0 as u8);
                }
                j = i;
                i = labels[i].1;
            }

            // erase all vertex labels except the ones of the source
            let source_label = labels[0];
            labels.clear();
            labels = vec![(0.0, 0, false); n];
            labels[0] = source_label;

            // reinitialize q with the source
            q.clear();
            q.push_back(0);
        }
    }

    x // the current flow is maximum
}

fn main() {
    // example network graph from figure 10.4
    let mut u = Compressed::<u8>::zero((6, 6));

    // 1
    u.set((0, 1), 2); // 1 -> 2: 2
    u.set((0, 3), 3); // 1 -> 4: 3

    // 2
    u.set((1, 2), 5); // 2 -> 3: 5
    u.set((1, 4), 3); // 2 -> 5: 3

    // 3
    u.set((2, 5), 2); // 3 -> 6: 2

    // 4
    u.set((3, 2), 1); // 4 -> 3: 1

    // 5
    u.set((4, 5), 4); // 5 -> 6: 4

    println!("Original graph (fig 10.4):");
    for i in 0..u.rows() {
        for j in 0..u.columns() {
            if u.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, u.get((i, j)));
            }
        }
    }

    let x = shortest_augmenting_path(&u);
    println!("Maximum flow graph:");
    for i in 0..x.rows() {
        for j in 0..x.columns() {
            if x.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, x.get((i, j)));
            }
        }
    }

    println!("\n====================================\n");

    // network graph from exercises 10.2 problem 2(a)
    let mut ex10_2_p_2a = Compressed::<u8>::zero((6, 6));

    // 1
    ex10_2_p_2a.set((0, 1), 5); // 1 -> 2: 5
    ex10_2_p_2a.set((0, 2), 6); // 1 -> 3: 6

    // 2
    ex10_2_p_2a.set((1, 3), 4); // 2 -> 4: 4
    ex10_2_p_2a.set((1, 4), 2); // 2 -> 5: 2

    // 3
    ex10_2_p_2a.set((2, 3), 7); // 3 -> 4: 7

    // 4
    ex10_2_p_2a.set((3, 5), 8); // 4 -> 6: 8

    // 5
    ex10_2_p_2a.set((4, 5), 4); // 5 -> 6: 4

    println!("Original graph (ex. 10.2 prob. 2(a)):");
    for i in 0..ex10_2_p_2a.rows() {
        for j in 0..ex10_2_p_2a.columns() {
            if ex10_2_p_2a.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, ex10_2_p_2a.get((i, j)));
            }
        }
    }

    let x_2 = shortest_augmenting_path(&ex10_2_p_2a);
    println!("Maximum flow graph:");
    for i in 0..x_2.rows() {
        for j in 0..x_2.columns() {
            if x_2.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, x_2.get((i, j)));
            }
        }
    }

    println!("\n====================================\n");

    // network graph from exercises 10.2 problem 2(b)
    let mut ex10_2_p_2b = Compressed::<u8>::zero((6, 6));

    // 1
    ex10_2_p_2b.set((0, 1), 2); // 1 -> 2: 2
    ex10_2_p_2b.set((0, 2), 7); // 1 -> 3: 7

    // 2
    ex10_2_p_2b.set((1, 3), 3); // 2 -> 4: 3
    ex10_2_p_2b.set((1, 4), 4); // 2 -> 5: 4

    // 3
    ex10_2_p_2b.set((2, 3), 4); // 3 -> 4: 4
    ex10_2_p_2b.set((2, 4), 2); // 3 -> 5: 2
    
    // 4
    ex10_2_p_2b.set((3, 5), 1); // 4 -> 6: 1

    // 5
    ex10_2_p_2b.set((4, 5), 5); // 5 -> 6: 5

    println!("Original graph (ex. 10.2 prob. 2(b)):");
    for i in 0..ex10_2_p_2b.rows() {
        for j in 0..ex10_2_p_2b.columns() {
            if ex10_2_p_2b.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, ex10_2_p_2b.get((i, j)));
            }
        }
    }

    let x_3 = shortest_augmenting_path(&ex10_2_p_2b);
    println!("Maximum flow graph:");
    for i in 0..x_3.rows() {
        for j in 0..x_3.columns() {
            if x_3.get((i, j)) != 0 {
                println!("\t{} -> {}: {}", i + 1, j + 1, x_3.get((i, j)));
            }
        }
    }
}
