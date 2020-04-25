use std::vec::Vec;
use std::cmp::Ordering;
use std::mem;

// courtesy of https://stackoverflow.com/a/28294764
fn swap<T>(x: &mut Vec<T>, i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        Ordering::Equal => return,

        // get the smallest and largest of the two indices
        Ordering::Less => (i, j),
        Ordering::Greater => (j ,i),
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

// Implements Johnson-Trotter algorithm for generating permutations
// Input: A positive integer n
// Output: A list of all permutations of {1, ..., n}
fn johnson_trotter(n: u8) {
    let mut m = Vec::<(u8, bool)>::new();

    // initialize first permutation with all arrows pointing left
    print!("\t");
    for i in 1..(n+1) {
        m.push((i, false));
        print!("{}", i);
    }
    print!("\n");

    // while the last permutation has a mobile element
    loop {
        let k = find_largest_mobile_element(&m);
        if k.1 {
            // no more mobile elements
            break;
        }

        // swap k with the adjacent element k's arrow points to
        let k_val = m[k.0].0;
        if m[k.0].1 {
            swap::<(u8, bool)>(&mut m, k.0, k.0 + 1);
        } else {
            swap::<(u8, bool)>(&mut m, k.0, k.0 - 1);
        }

        // reverse the direction of all the elements that are larger than k
        for i in 0..m.len() {
            if m[i].0 > k_val {
                m[i].1 ^= !m[i].1;
            }
        }

        // add the new permutation to the list
        print!("\t");
        for j in 0..m.len() {
            print!("{}", m[j].0);
        }
        print!("\n");
    }
}

fn find_largest_mobile_element(v: &Vec<(u8, bool)>) -> (usize, bool) {
    // find the permutation's largest mobile element k
    let mut k_val: u8 = 0;
    let mut k: usize = 0;

    for i in 0..v.len() {
        if v[i].0 > k_val {
            if v[i].1 {
                // arrow pointing to the right
                if (i != (v.len() - 1)) && v[i].0 > v[i + 1].0 {
                    k = i;
                    k_val = v[i].0;
                }
            } else {
                // arrow pointing to the left
                if (i != 0) && v[i].0 > v[i - 1].0 {
                    k = i;
                    k_val = v[i].0;
                }
            }
        }
    }
    
    // if k_val == 0, no mobile elements remain
    (k, k_val == 0)
}

fn main() {
    println!("All permutation for n = 4:");
    johnson_trotter(4);
}
