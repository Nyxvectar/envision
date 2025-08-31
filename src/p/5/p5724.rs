/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/31/2025
 */

use std::io;

fn main() {
    let mut n = get_input(1)[0];
    let mut k = get_input(n);
    k.sort();
    print!("{}", k[(n - 1) as usize] - k[0])
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
