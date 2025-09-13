/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let k = input(2);
    let mut sum = 0;
    for i in 1..=k[0] {
        let mut temp = i;
        while temp != 0 {
            if (temp % 10) == k[1] {
                sum += 1;
            }
            temp /= 10;
        }
    }
    println!("{}", sum);
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
