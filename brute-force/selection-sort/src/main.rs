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

// Sorts a given array by selection sort
// Input: An array a[0..n-1] of orderable elements
// Output: Array a[0..n-1] sorted in nondecreasing order
fn selection_sort(a: &mut [u8]) {
    let n = a.len();
    for i in 0..(n - 1) {
        let mut min = i;
        for j in (i + 1)..n {
            if a[j] < a[min] {
                min = j;
            }
        }
        swap::<u8>(a, i, min);
    }
}

fn main() {
    // sort example array from book
    let mut a: [u8; 7] = [89, 45, 68, 90, 29, 34, 17];
    println!("Array before selection sort: {:?}", a);

    // should be [17, 29, 34, 45, 68, 89, 90]
    selection_sort(&mut a);
    println!("Array after selection sort: {:?}", a);
}
