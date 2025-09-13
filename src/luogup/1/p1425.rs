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
    let (a, b, c, d) = (needs[0], needs[1], needs[2], needs[3]);
    let mut minutes = 60 - b + d;
    let mut hours = c - a - 1;
    if minutes >= 60 {
        hours += 1;
        minutes -= 60;
    }
    print!("{} {}", hours, minutes)
}
