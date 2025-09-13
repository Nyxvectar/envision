/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (a, b, c) = (get[0], get[1], get[2]);
    let p = (a + b + c) / 2.0;
    let s = (p * (p - a) * (p - b) * (p - c)).sqrt();
    print!("{:.1}", s);
}
