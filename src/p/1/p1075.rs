/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 09/06/2025
 */

use std::io;

fn main() {
    let get = input(1)[0];
    let mut i = 2;
    loop {
        if get % i == 0 {
            break;
        } else {
            i += 1;
        }
    }
    print!("{}", get / i)
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
