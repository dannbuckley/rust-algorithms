// Computes a^n by the right-to-left binary exponentiation algorithm
// Input: A number a and a list b(n) of binary digits b_I, ..., b_0
//     in the binary expansion of a nonnegative integer n
// Output: The value of a^n
fn rl_binary_exponentiation(a: u8, b: &[u8]) -> u64 {
    let mut term: u64 = a as u64; // initializes a^(2^i)
    let i = b.len();
    let mut product: u64;

    if b[i - 1] == 1 {
        product = a as u64;
    } else {
        product = 1;
    }

    for j in 1..i {
        term = term * term;
        if b[i - 1 - j] == 1 {
            product = product * term;
        }
    }

    product
}

fn main() {
    let b = [1, 1, 0, 1];
    println!("Value of 5^13 (RL): {}", rl_binary_exponentiation(5, &b));
}
