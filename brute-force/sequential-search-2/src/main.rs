use std::cmp::Ordering;
use std::vec::Vec;

// Implements sequential search with a search key as a sentinel
// Input: An array a of n elements and a search key k
// Output: The index of the first element in a[0..n-1] whose value is equal to k
//    or -1 if no such element is found
fn sequential_search_2(a: &[u8], k: u8) -> i8 {
    let n = a.len();
    let mut v: Vec<u8> = a.to_vec();
    v.push(k);
    let mut i = 0;
    while v[i] != k {
        i = i + 1;
    }
    let ret: i8 = match i.cmp(&n) {
        Ordering::Equal => -1,
        Ordering::Less => i as i8,
        Ordering::Greater => -1,
    };
    ret
}

fn main() {
    let a: [u8; 7] = [89, 45, 68, 90, 29, 34, 17];
    println!("Array to search: {:?}", a);
    println!("Index of '90': {}", sequential_search_2(&a, 90));
    println!("Index of '20': {}", sequential_search_2(&a, 20));
}
