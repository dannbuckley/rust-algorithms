use std::string::String;
use std::vec::Vec;

// t(c) = {
//     the pattern's length m,
//       if c is not among the first m - 1 characters of the pattern;
//     the distance from the rightmost c among the first m - 1 characters
//       of the pattern to its last character, otherwise.
// }

// Fills the shift table used by Horspool's and Boyer-Moore algorithms
// Input: Pattern p[0..m-1] and an alphabet of possible characters
// Output: Table[0..size-1] indexed by the alphabet's characters and filled with
//     shift sizes computed by t(c)
fn shift_table(p: &String, a: &Vec<char>) -> Vec<usize> {
    let p_ = p.as_bytes();
    let m = p.len();
    let size = a.len();
    let mut table = Vec::<usize>::new();

    for _ in 0..size {
        table.push(m);
    }

    for j in 0..(m - 1) {
        let t_index = a.binary_search(&(p_[j] as char)).unwrap();
        table[t_index] = m - 1 - j;
    }

    table
}

// Implements Horspool's algorithm for string matching
// Input: Pattern p[0..m-1] and text t[0..n-1]
// Output: The index of the left end of the first matching substring
//     or -1 if there are no matches
fn horspool_matching(p: &String, t: &String) -> isize {
    // generate table of shifts
    let a = vec![
        'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y',
        'Z', '_'];
    let table = shift_table(p, &a);
    let n = t.len();
    let m = p.len();
    let p_ = p.as_bytes();
    let t_ = t.as_bytes();
    let mut i = m - 1; // position of the pattern's right end

    while i <= n - 1 {
        let mut k = 0; // number of match characters
        while k <= m - 1 && p_[m - 1 - k] == t_[i - k] {
            k = k + 1;
        }
        if k == m {
            return (i - m + 1) as isize;
        } else {
            let t_index = a.binary_search(&(t_[i] as char)).unwrap();
            i = i + table[t_index];
        }
    }
    
    -1
}

fn main() {
    let p = String::from("BARBER");
    let t = String::from("JIM_SAW_ME_IN_A_BARBERSHOP");
    println!("Pattern: {}", p);
    println!("Text: {}", t);
    println!("Index of pattern in text: {}", horspool_matching(&p, &t));
}
