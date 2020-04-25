// Finds distance between two closest points in the plane by brute force
// Input: A list p of n (n >= 2) points p_1(x_1, y_1), ..., p_n(x_n, y_n)
// Output: The distance between the closest pair of points
fn brute_force_closest_pair(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    let mut d = f64::INFINITY;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let xd = p[i].0 - p[j].0;
            let yd = p[i].1 - p[j].1;
            let e = xd.powi(2) + yd.powi(2);
            d = d.min(e.sqrt());            
        }
    }
    d
}

fn main() {
    let p: [(f64, f64); 4] = [(0f64, 0f64), (7f64, 3f64) ,(1f64, 1f64), (2f64, 5f64)];
    println!("Points: {:?}", p);    
    println!("Distance between closest pair of points: {}", brute_force_closest_pair(&p));
}
