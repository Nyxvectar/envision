/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let sqrt5 = 5.0_f64.sqrt();
    let a = (1.0 + sqrt5) / 2.0;
    let b = (1.0 - sqrt5) / 2.0;
    let fn_value = (a.powi(n as i32) - b.powi(n as i32)) / sqrt5;
    println!("{:.2}", fn_value);
}
