extern crate priority_queue;

use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::string::String;
use std::vec::Vec;

// Implements Huffman's algorithm for variable-length encoding
// Input: Table f[0..n-1] of symbols and their associated frequencies
// Output: A hashmap h<K, V> with the symbol as the key and the codeword as the value
fn huffman(f: &Vec<(&str, u8)>) -> HashMap<char, String> {
    // initialize n one-node trees and label them with the symbols of the alphabet given
    let mut q = PriorityQueue::<(String, VecDeque<String>), u8>::new();
    for c in f {
        // initialize queue to store codewords
        let mut cd = VecDeque::<String>::new();
        cd.push_back(String::from(""));

        // push tree to priority queue with inverse frequency
        q.push((String::from(c.0), cd), 100 - c.1);
    }

    // repeat until there is a single tree
    while q.len() > 1 {
        // left subtree of new tree
        let l = q.pop().unwrap();
        let l_symbols = (l.0).0;
        let mut l_q = (l.0).1;

        // right subtree of new tree
        let r = q.pop().unwrap();
        let r_symbols = (r.0).0;
        let mut r_q = (r.0).1;

        let mut root_codes = VecDeque::<String>::new();

        // label left edge with "0"
        while !l_q.is_empty() {
            let mut left_code = String::from("0");
            left_code.push_str(l_q.pop_front().unwrap().as_str());
            root_codes.push_back(left_code);
        }

        // label right edge with "1"
        while !r_q.is_empty() {
            let mut right_code = String::from("1");
            right_code.push_str(r_q.pop_front().unwrap().as_str());
            root_codes.push_back(right_code);
        }

        // combine symbols of left and right subtree
        let mut root_symbols = l_symbols;
        root_symbols.push_str(r_symbols.as_str());

        // compute new priority
        let p = l.1 + r.1 - 100;

        // push new tree to priority queue
        q.push((root_symbols, root_codes), p);
    }

    let end_tree = q.pop().unwrap().0;
    let mut et_symbols = end_tree.0;
    let mut et_codes = end_tree.1;

    // construct hashmap from Huffman tree
    let mut h = HashMap::<char, String>::new();
    while !et_symbols.is_empty() && !et_codes.is_empty() {
        h.insert(et_symbols.pop().unwrap(), et_codes.pop_back().unwrap());
    }

    h
}

fn main() {
    // use example from book with probabilities as whole-number percents
    let mut f = Vec::<(&str, u8)>::new();
    f.push(("A", 35));
    f.push(("B", 10));
    f.push(("C", 20));
    f.push(("D", 20));
    f.push(("_", 15));

    println!("Initial alphabet and frequencies:");
    for i in 0..f.len() {
        println!("\t{}:\t{}", f[i].0, f[i].1 as f64 / 100.0);
    }

    let h = huffman(&f);

    println!("Generated Huffman codewords:");

    for codeword in h {
        println!("\t{}:\t{}", codeword.0, codeword.1);
    }
}
