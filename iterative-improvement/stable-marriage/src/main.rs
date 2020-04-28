extern crate matrix;

use matrix::prelude::*;
use std::collections::VecDeque;
use std::vec::Vec;

fn stable_marriage(m: &Compressed<u8>, f: &Compressed<u8>) -> Vec<(usize, usize)> {
    let mut pairs = Vec::<(usize, usize)>::new();

    // start with all the men and women being free
    // m.dimensions() == f.dimensions() == (n x n), so either works
    let mut matches = Compressed::<u8>::zero(m.dimensions());
    let mut free_men = VecDeque::<usize>::new();
    for i in 0..m.rows() {
        free_men.push_back(i);
    }

    // while there are free men, arbitrarily select one of them
    while !free_men.is_empty() {
        let b = free_men.pop_front().unwrap();
        let mut b_paired = false;
        for w_index in 0..m.columns() {
            // proposal
            let w = m.get((b, w_index)) as usize;
            let mut w_free = true;
            let mut w_mate = 0;

            while w_free && w_mate < matches.columns() {
                if matches.get((w_mate, w)) != 0 {
                    w_free = false;
                } else {
                    w_mate = w_mate + 1;
                }
            }

            // response
            if w_free {
                matches.set((b, w), 1);
                b_paired = true;
                break;
            } else {
                let mut w_mate_rank = 0;
                let mut w_b_rank = 0;

                // get w's ranking for b and previous mate
                for i in 0..f.columns() {
                    let w_i = f.get((w, i));
                    if w_i == w_mate as u8 {
                        w_mate_rank = i + 1;
                    }
                    else if w_i == b as u8 {
                        w_b_rank = i + 1;
                    }
                }

                // if w prefers b to previous mate, previous mate is now free
                if w_b_rank < w_mate_rank {
                    matches.set((b, w), 1);
                    b_paired = true;                    
                    matches.set((w_mate, w), 0);
                    free_men.push_back(w_mate);
                    break;
                }
            }
        }

        if !b_paired {
            free_men.push_back(b);
        }
    }

    // reduce matrix to pairs
    for i in 0..matches.rows() {
        for j in 0..matches.columns() {
            if matches.get((i, j)) != 0 {
                pairs.push((i, j));
                break;
            }
        }
    }

    pairs
}

fn main() {
    // use example from figure 10.11 in book
    // Men: Bob, Jim, Tom
    let mut m = Compressed::<u8>::zero((3, 3));

    // Bob
    m.set((0, 0), 1); // 1st: Lea
    m.set((0, 1), 0); // 2nd: Ann
    m.set((0, 2), 2); // 3rd: Sue

    // Jim
    m.set((1, 0), 1); // 1st: Lea
    m.set((1, 1), 2); // 2nd: Sue
    m.set((1, 2), 0); // 3rd: Ann

    // Tom
    m.set((2, 0), 2); // 1st: Sue
    m.set((2, 1), 1); // 2nd: Lea
    m.set((2, 2), 0); // 3rd: Ann

    println!("Male preference matrix:");
    for i in 0..m.rows() {
        for j in 0..m.columns() {
            print!("\t{}", m.get((i, j)));            
        }
        print!("\n");
    }

    // Women: Ann, Lea, Sue
    let mut f = Compressed::<u8>::zero((3, 3));

    // Ann
    f.set((0, 0), 1); // 1st: Jim
    f.set((0, 1), 2); // 2nd: Tom
    f.set((0, 2), 0); // 3rd: Bob

    // Lea
    f.set((1, 0), 2); // 1st: Tom
    f.set((1, 1), 0); // 2nd: Bob
    f.set((1, 2), 1); // 3rd: Jim

    // Sue
    f.set((2, 0), 1); // 1st: Jim
    f.set((2, 1), 2); // 2nd: Tom
    f.set((2, 2), 0); // 3rd: Bob

    println!("Female preference matrix:");
    for i in 0..f.rows() {
        for j in 0..f.columns() {
            print!("\t{}", f.get((i, j)));            
        }
        print!("\n");
    }

    let pairs = stable_marriage(&m, &f);

    println!("Matched pairs:");
    for pair in pairs {
        println!("\t({}, {})", pair.0, pair.1);
    }
}
