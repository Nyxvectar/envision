/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let mut a = get_input(1)[0];
    let mut counter = 1;
    loop {
        a /= 2;
        counter += 1;
        if a == 1 {
            break;
        }
    }
    print!("{}", counter)
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
