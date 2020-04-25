// Sorts a given array by insertion sort
// Input: An array a[0..n-1] of n orderable elements
// Output: Array a[0..n-1] sorted in nondecreasing order
// Note: To avoid encountering negative indices, this implementation
//     sorts the array right-to-left whereas the original algorithm
//     sorts left-to-right
fn insertion_sort(a: &mut [u8]) {
    let n = a.len();
    for i in 1..n {
        let v = a[n - 1 - i];
        let mut j = n - i;
        while j < n && a[j] < v {
            a[j - 1] = a[j];
            j = j + 1;
        }
        a[j - 1] = v;
    }
}

fn main() {
    // sort example array from book
    let mut a: [u8; 7] = [89, 45, 68, 90, 29, 34, 17];
    println!("Array before insertion sort: {:?}", a);

    // should be [17, 29, 34, 45, 68, 89, 90]
    insertion_sort(&mut a);
    println!("Array after insertion sort: {:?}", a);
}
