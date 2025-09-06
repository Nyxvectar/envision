/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let n = input(1)[0];
    let total = n / 364;
    for k in 1..(total / 3 + 1) {
        let x = total - 3 * k;
        if x > 0 && x <= 100 {
            println!("{}", x);
            println!("{}", k);
            return;
        }
    }
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
