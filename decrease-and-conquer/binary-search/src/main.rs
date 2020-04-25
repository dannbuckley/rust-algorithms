extern crate math;

use math::round;

// Implements nonrecursive binary search
// Input: An array a[0..n-1] sorted in ascending order and a search key k
// Output: An index of the array's element that is equal to k
//     or -1 if there is no such element
fn binary_search(a: &[u8], k: u8) -> isize {
    let n = a.len();
    let mut l: usize = 0;
    let mut r: usize = n - 1;
    let mut ret: isize = -1;

    while l <= r {
        let m = round::floor((l + r) as f64 / 2.0, 0) as usize;

        if k == a[m] {
            ret = m as isize;
            break;
        }
        else if k < a[m] {
            r = m - 1;
        }
        else {
            l = m + 1;
        }
    }

    ret
}

fn main() {
    // search example array from book
    let a: [u8; 13] = [3, 14, 27, 31, 39, 42, 55, 70, 74, 81, 85, 93, 98];
    println!("Array to search: {:?}", a);
    println!("Location of key 70 within array: {}", binary_search(&a, 70));
}
