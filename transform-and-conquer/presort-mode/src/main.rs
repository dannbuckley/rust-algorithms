extern crate math;

use math::round;
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

// Constructs a heap from elements of a given array by the bottom-up algorithm
// Input: An array h[1..n] of orderable items
// Output: A heap h[1..n]
fn heap_bottom_up(h: &mut Vec<u8>) {
    let n = h.len();
    let nh = round::floor(n as f64 / 2.0, 0) as usize;
    for i in 1..(nh + 1) {
        let mut k = nh - i + 1;
        let v = h[k - 1];
        let mut heap = false;
        while !heap && (2 * k) <= n {
            let mut j = 2 * k;
            if j < n {
                // there are two children
                if h[j - 1] < h[j] {
                    j = j + 1;
                }
            }
            if v >= h[j - 1] {
                heap = true;
            } else {
                h[k - 1] = h[j - 1];
                k = j;
            }
        }
        h[k - 1] = v;
    }
}

fn heapsort(h: &mut Vec<u8>) {
    // construct a heap for a given array
    heap_bottom_up(h);

    // apply the root-deletion operation n-1 times to the remaining heap
    let n = h.len();
    for i in 0..(n-1) {
        // exchange the root's key with the last key k of the heap
        swap::<u8>(h, 0, n - 1 - i);

        // verify the parental dominance for k
        let mut hn = Vec::<u8>::from(&h[0..(n - 1 - i)]);
        heap_bottom_up(&mut hn);
        for j in 0..(n-1-i) {
            h[j] = hn[j];
        }
    }
}

// Computes the mode of an array by sorting it first
// Input: An array a[0..n-1] of orderable elements
// Output: The array's mode
fn presort_mode(a: &mut Vec<u8>) -> u8 {
    heapsort(a);

    let n = a.len();
    let mut i: usize = 0; // current run begins at position i
    let mut mode_frequency = 0; // highest frequency seen so far
    let mut mode_value = 0;
    while i <= n - 1 {
        let mut run_length = 1;
        let run_value = a[i];

        while (i + run_length <= n - 1) && (a[i + run_length] == run_value) {
            run_length = run_length + 1;
        }

        if run_length > mode_frequency {
            mode_frequency = run_length;
            mode_value = run_value;
        }

        i = i + run_length;
    }

    mode_value
}

fn main() {
    let mut a = vec![13, 11, 12, 13, 12, 12];
    println!("Array to search: {:?}", a);
    println!("Array mode: {}", presort_mode(&mut a));
}
