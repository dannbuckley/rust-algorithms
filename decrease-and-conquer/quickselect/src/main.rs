use std::cmp::Ordering;
use std::mem;

// courtesy of https://stackoverflow.com/a/28294764
fn swap<T>(x: &mut [T], i: usize, j: usize) {
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

// Partitions subarray by Lomuto's algorithm using first element as pivot
// Input: A subarray a[l..r] of array a[0..n-1], defined by
//     its left and right indices l and r (l <= r)
// Output: Partition of a[l..r] and the new position of the pivot
fn lomuto_partition(a: &mut [u8], l: usize, r: usize) -> usize {
    let p = a[l];
    let mut s = l;

    for i in (l + 1)..(r + 1) {
        if a[i] < p {
            s = s + 1;
            swap(a, s, i);
        }
    }
    swap(a, l, s);

    s
}

// Solves the selection problem by recursive partition-based algorithm
// Input: Subarray a[l..r] of array a[0..n-1] of orderable elements
//     and integer k (1 <= k <= r - l + 1)
// Output: The value of the kth smallest element in a[l..r]
fn quickselect(a: &mut [u8], l: usize, r: usize, k: usize) -> u8 {
    let s = lomuto_partition(a, l, r);
    
    if s == k - 1 {
        return a[s];    
    }
    else if s > (l + k - 1) {
        return quickselect(a, l, s - 1, k);
    }
    else {
        return quickselect(a, s + 1, r, k);
    }
}

fn main() {
    // search example array from book
    let mut a: [u8; 9] = [4, 1, 10, 8, 7, 12, 9, 2, 15];
    println!("Array to search: {:?}", a);
    println!("The 5th smallest element of the array: {}", quickselect(&mut a, 0, 8, 5));
}
