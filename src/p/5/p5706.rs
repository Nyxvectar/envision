/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/23/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<f32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (t, n) = (get[0], get[1]);

    println!("{:.3}", t / n);
    print!("{}", n * 2.0)
}
