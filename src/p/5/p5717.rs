/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/24/2025
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
    let (a, b, c) = (get[0], get[1], get[2]);
    if a + b <= c || a + c <= b || b + c <= a || a <= 0.0 || b <= 0.0 || c <= 0.0 {
        println!("Not triangle");
    } else {
        if a * a + b * b == c * c || a * a + c * c == b * b || b * b + c * c == a * a {
            println!("Right triangle");
        } else if (a * a + b * b - c * c) / (2.0 * a * b) < 0.0
            || (b * b + c * c - a * a) / (2.0 * b * c) < 0.0
            || (a * a + c * c - b * b) / (2.0 * a * c) < 0.0
        {
            println!("Obtuse triangle");
        } else {
            println!("Acute triangle");
        }
        if a == b || b == c || a == c {
            println!("Isosceles triangle");
        }
        if a == b && a == c {
            println!("Equilateral triangle");
        }
    }
}
