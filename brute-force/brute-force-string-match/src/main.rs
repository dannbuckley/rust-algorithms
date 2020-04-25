use std::string::String;

// Implements brute-force string matching
// Input: An array t[0..n-1] of n characters representing a text
//    and an array p[0..m-1] of m characters representing a pattern
// Output: The index of the first character in the text that starts a matching substring
//    or -1 if the search is unsuccessful
fn brute_force_string_match(t: String, p: String) -> i8 {
    let n = t.len();
    let m = p.len();
    let mut ret: i8 = -1;
    for i in 0..(n-m) {
        let mut j = 0;
        while (j < m) && (p.as_bytes()[j] == t.as_bytes()[i + j]) {
            j = j + 1;
        }
        if j == m {
            ret = i as i8;
            break;
        }
    }
    ret
}

fn main() {
    // run string match algorithm on book example
    let t = String::from("NOBODY_NOTICED_HIM");
    let p = String::from("NOT");
    println!("Text: {}", t);
    println!("Pattern: {}", p);
    println!("Index of first instance of pattern in text: {}", brute_force_string_match(t, p));
}
