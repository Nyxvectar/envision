/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let get = get_input(1)[0];
    let local = 5 * get;
    let remote = 3 * get + 11;
    if local < remote {
        print!("Local")
    } else {
        print!("Luogu")
    }
}

fn get_input(how_many_need: i64) -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}
