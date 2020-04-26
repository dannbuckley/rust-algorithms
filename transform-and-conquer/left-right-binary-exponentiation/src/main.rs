// Applies Horner's rule to exponentiation
// Computes a^n by the left-to-right binary exponentiation algorithm
// Input: A number a and a list b(n) of binary digits b_I, ..., b_0
//     in the binary expansion of a positive integer n
// Output: The value of a^n
fn lr_binary_exponentiation(a: u8, b: &[u8]) -> u64 {
    let mut product: u64 = a as u64;
    let i = b.len();

    for j in 0..(i - 1) {
        product = product * product;
        if b[i - 1 - j] == 1 {
            product = product * (a as u64);
        }
    }

    product
}

fn main() {
    let b = [1, 1, 0, 1];
    println!("Value of 5^13 (LR): {}", lr_binary_exponentiation(5, &b));
}
