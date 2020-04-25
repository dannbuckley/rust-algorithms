extern crate math;

use math::round;
use std::vec::Vec;

// Sorts array a[0..n-1] by recursive mergesort
// Input: An array a[0..n-1] of orderable elements
// Output: Array a[0..n-1] sorted in nondecreasing order
fn mergesort(a: &mut Vec<u8>) {
    let n = a.len();
    if n > 1 {
        let nh: usize = round::floor(n as f64 / 2.0, 0) as usize;
        let mut b = Vec::from(&a[0..nh]);        
        let mut c = Vec::from(&a[nh..n]);
        mergesort(&mut b);
        mergesort(&mut c);
        merge(&b, &c, a);
    }
}

// Merges two sorted array into one sorted array
// Input: Arrays b[0..p-1] and c[0..q-1] both sorted
// Output: Sorted array a[0..p+q-1] of the elements of b and c
fn merge(b: &Vec<u8>, c: &Vec<u8>, a: &mut Vec<u8>) {
    let p = b.len();
    let q = c.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < p && j < q {
        if b[i] <= c[j] {
            a[k] = b[i];
            i = i + 1;            
        } else {
            a[k] = c[j];
            j = j + 1;
        }
        k = k + 1;
    }

    if i == p {
        for x in j..q {
            a[(x - j) + k] = c[x];
        }
    } else {
        for x in i..p {
            a[(x - i) + k] = b[x];
        }
    }
}

fn main() {
    // sort example array from book
    let mut a: Vec<u8> = vec![8, 3, 2, 9, 7, 1, 5, 4];
    println!("Array before mergesort: {:?}", a);

    mergesort(&mut a);    
    println!("Array after mergesort: {:?}", a);
}
