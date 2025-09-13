/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/23/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let needs: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (a, b) = (needs[0], needs[1]);
    println!("{}", (a * 10 + b) / 19)
}
