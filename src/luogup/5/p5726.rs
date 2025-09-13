/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let get = input(1)[0];
    let mut list = input(get);
    list.sort();
    let mut sum = 0;
    for i in 1..get - 1 {
        sum += &list[i as usize];
    }
    print!("{:.2}", sum as f64 / (get as f64 - 2f64))
}

fn input(how_many_need: i64) -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}
