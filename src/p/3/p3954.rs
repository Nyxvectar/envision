/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (a, b, c) = (get[0], get[1], get[2]);
    let grade = (a * 2 + b * 3 + c * 5) / 10;
    println!("{}", grade)
}
