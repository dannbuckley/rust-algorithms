// Evaluates a polynomial at a given point by Horner's rule
// Input: An array p[0..n] of coefficients of a polynomial of degree n,
//     stored from the lowest to the highest and a number x
// Output: The value of the polynomial at x
fn horner(p: &[i8], x: i8) -> i16 {
    let n = p.len();
    let mut pv: i16 = p[n - 1] as i16;

    for i in 0..(n-1) {
        pv = (x as i16 * pv) + p[n - 2 - i] as i16;
    }

    pv
}

fn main() {
    // evaluate example polynomial from book
    let p = [-5, 1, 3, -1, 2];
    println!("Evaluation of 2x^4 - x^3 + 3x^2 + x - 5 at x = 3: {}", horner(&p, 3));
}
