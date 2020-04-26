use std::vec::Vec;

// Sorts an array by comparison counting
// Input: An array a[0..n-1] of orderable elements
// Output: Array s[0..n-1] of a's elements sorted in nondecreasing order
fn comparison_counting_sort(a: &Vec<u8>) -> Vec<u8> {
    let n = a.len();
    let mut s = Vec::<u8>::new();
    let mut count = Vec::<u8>::new();

    for _ in 0..n {
        count.push(0);
        s.push(0);
    }
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if a[i] < a[j] {
                count[j] = count[j] + 1;
            } else {
                count[i] = count[i] + 1;
            }
        }
    }
    for i in 0..n {
        s[count[i] as usize] = a[i];
    }

    s
}

fn main() {
    // sort example array from book
    let a = vec![62, 31, 84, 96, 19, 47];
    println!("Array before comparison counting sort: {:?}", a);
    println!("Array after comparison counting sort: {:?}", comparison_counting_sort(&a));
}
