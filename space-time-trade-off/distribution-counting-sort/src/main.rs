use std::vec::Vec;

// Sorts an array of integers from a limited range by distribution counting
// Input: An array a[0..n-1] of integers between l and u (l <= u)
// Output: Array s[0..n-1] of a's elements sorted in nondecreasing order
fn distribution_counting_sort(a: &Vec<u8>, l: u8, u: u8) -> Vec<u8> {
    let n = a.len();
    let mut d = Vec::<u8>::new();
    let mut s = Vec::<u8>::new();

    for _ in 0..(u - l + 1) {
        d.push(0); // initialize frequencies
    }
    for i in 0..n {
        d[(a[i] - l) as usize] = d[(a[i] - l) as usize] + 1; // compute frequencies
        s.push(0);
    }
    for j in 1..(u - l + 1) {
        d[j as usize] = d[(j - 1) as usize] + d[j as usize]; // reuse for distribution
    }
    for i in 0..n {
        let j = (a[n - 1 - i] - l) as usize;
        s[(d[j] - 1) as usize] = a[n - 1 - i];
        d[j] = d[j] - 1;
    }

    s
}

fn main() {
    // sort example array from book
    let a = vec![13, 11, 12, 13, 12, 12];
    println!("Array before distribution counting sort: {:?}", a);
    println!("Array after distribution counting sort: {:?}", distribution_counting_sort(&a, 11, 13));
}
