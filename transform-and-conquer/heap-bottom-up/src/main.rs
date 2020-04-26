extern crate math;

use math::round;

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

fn main() {
    // construct heap from example array in book
    let mut h = vec![2, 9, 7, 6, 5, 8];
    println!("Array before heap construction: {:?}", h);

    heap_bottom_up(&mut h);
    println!("Array after heap construction: {:?}", h);
}
