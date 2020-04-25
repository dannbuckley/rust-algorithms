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

// Sorts a subarray by quicksort
// Input: Subarray of array a[0..n-1], defined by its left and right indices l and r
// Output: Subarray a[l..r] sorted in nondecreasing order
fn quicksort(a: &mut [u8], l: usize, r: usize) {
    if l < r {
        let s = hoare_partition(a, l, r);
        quicksort(a, l, s - 1);
        quicksort(a, s + 1, r);
    }
}

// Partitions a subarray by Hoare's algorithm, using the first element as a pivot
// Input: Subarray of array a[0..n-1], defined by its left and right indices l and r (l < r)
// Output: Partition of a[l..r], with the split position returned as this function's value
fn hoare_partition(a: &mut [u8], l: usize, r: usize) -> usize {
    let p = a[l];
    let mut i = l;
    let mut j = r + 1;

    while i < j {
        loop {
            i = i + 1;
            if a[i] >= p {
                break;
            }
        }
        loop {
            j = j - 1;
            if a[j] <= p {
                break;
            }
        }
        swap(a, i, j);
    }

    swap(a, i, j); // undo last swap when i >= j
    swap(a, l, j);

    j
}

fn main() {
    // sort example array from book
    let mut a: [u8; 8] = [5, 3, 1, 9, 8, 2, 4, 7];
    println!("Array before quicksort: {:?}", a);

    quicksort(&mut a, 0, 7);
    println!("Array after quicksort: {:?}", a);
}
