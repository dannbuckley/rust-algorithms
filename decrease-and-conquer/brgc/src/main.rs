use std::string::String;
use std::vec::Vec;

// Generates recursively the binary reflected Gray code of order n
// Input: A positive integer n
// Output: A list of all bit strings of length n composing the Gray code
fn brgc(n: u8) -> Vec<String> {
    let mut l = Vec::<String>::new();

    if n == 1 {
        l.push(String::from("0"));
        l.push(String::from("1"));
    } else {
        // generate list L1 of bit strings of size n - 1 by calling brgc(n - 1)
        let mut l1 = brgc(n - 1);
        let mut l2 = Vec::<String>::new();

        // copy list L1 to list L2 in reversed order
        let sn = l1.len();
        for k in 0..sn {
            l2.push(l1[sn - 1 - k].clone());
        }

        // add 0 in front of each bit string in list L1
        for i in 0..l1.len() {
            let mut newstr = "0".to_owned();
            newstr.push_str(l1[i].as_str());
            l1[i] = newstr;
        }

        // add 1 in front of each bit string in list L2
        for j in 0..l2.len() {
            let mut newstr = "1".to_owned();
            newstr.push_str(l2[j].as_str());
            l2[j] = newstr;
        }

        // append L2 to L1 to get list L
        l.extend(l1.into_iter());
        l.extend(l2.into_iter());
    }

    l
}

fn main() {
    println!("BRGC of orders n = 1, 2, 3:");
    println!("\t{:?}", brgc(1));
    println!("\t{:?}", brgc(2));
    println!("\t{:?}", brgc(3));
}
