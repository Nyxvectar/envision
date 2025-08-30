/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/30/2025
 */

use std::io;

fn main() {
    let mut get = get_input(3);
    get.sort();
    let (a, b, c) = (get[0], get[1], get[2]);
    let (sin1, sin2) = (a as f64 / c as f64, b as f64 / c as f64);
    if sin1 > sin2 {
        let (upper, lower) = simplfied(b, c);
        print!("{}/{}", upper, lower);
    } else {
        let (upper, lower) = simplfied(a, c);
        print!("{}/{}", upper, lower);
    }
}

fn simplfied(mut upper: i64, mut lower: i64) -> (i64, i64) {
    for i in 2..8 {
        if upper % i == 0 && lower % i == 0 {
            (upper, lower) = (upper / i, lower / i)
        }
    }
    (upper, lower)
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
