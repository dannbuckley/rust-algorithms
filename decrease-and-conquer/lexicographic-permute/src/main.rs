use std::vec::Vec;
use std::cmp::Ordering;
use std::mem;

// courtesy of https://stackoverflow.com/a/28294764
fn swap<T>(x: &mut Vec<T>, i: usize, j: usize) {
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

// Generates permutations in lexicographic order
// Input: A positive integer n
// Output: A list of all permutations of {1, ..., n} in lexicographic order
fn lexicographic_permute(n: u8) {
    let mut a = Vec::<u8>::new();

    // initialize the first permutation with 12...n
    print!("\t");
    for i in 1..(n+1) {
        a.push(i);
        print!("{}", i);
    }
    print!("\n");

    // while the last permutation has two consecutive elements in increasing order
    loop {
        // let i be its largest index such that a[i] < a[i + 1]
        // a[i + 1] > a[i + 2] > ... > a[n]
        let i = find_largest_i(&a);
        if i.1 {
            // no more valid elements
            break;
        }

        // find the largest index j such that a[i] < a[j]
        // j >= i + 1 since a[i] < a[i + 1]
        let j = find_largest_j(&a, i.0);
        if j.1 {
            // no more valid elements
            break;
        }

        // swap a[i] with a[j]
        // a[i + 1]a[i + 2]...a[n]
        swap::<u8>(&mut a, i.0, j.0);

        // reverse the order of the elements from a[i + 1] to a[n] inclusive
        let diff = ((n as usize) - 1) - (i.0 + 1);
        if diff > 0 {
            let a_t = a.to_vec();
            for k in 0..(diff + 1) {
                a[i.0 + 1 + k] = a_t[(n as usize) - 1 - k];
            }
        }

        // add the new permutation to the list
        print!("\t");
        for k in 0..(n as usize) {
            print!("{}", a[k]);
        }
        print!("\n");
    }
}

fn find_largest_i(a: &Vec<u8>) -> (usize, bool) {
    let n = a.len();
    let mut i_val: u8 = 0;
    let mut i: usize = 0;

    for k in 0..(n - 1) {
        if a[k] < a[k + 1] && i < k {
            i_val = a[k];
            i = k;
        }
    }

    (i, i_val == 0)
}

fn find_largest_j(a: &Vec<u8>, i: usize) -> (usize, bool) {
    let n = a.len();
    let mut j_val: u8 = 0;
    let mut j: usize = 0;

    for k in (i + 1)..n {
        if a[i] < a[k] && j < k {
            j_val = a[k];
            j = k;
        }
    }

    (j, j_val == 0)
}

fn main() {
    println!("Lexicographic permutations for n = 4:");
    lexicographic_permute(4);
}
