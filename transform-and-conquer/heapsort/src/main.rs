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

fn main() {
    // sort example array from book
    let mut h = vec![2, 9, 7, 6, 5, 8];
    println!("Array before heapsort: {:?}", h);

    heapsort(&mut h);
    println!("Array after heapsort: {:?}", h);
}
