/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::arch::x86_64::_xgetbv;
use std::cmp::max;
use std::io;

fn main() {
    let mut values = get_input(3);
    values.sort();
    let (a, b, c) = (values[0], values[1], values[2]);
    let orders = get_string_input();
    for k in 0..3 {
        match orders.chars().nth(k).unwrap() {
            'A' => {
                print!("{} ", a)
            }
            'B' => {
                print!("{} ", b)
            }
            _ => {
                print!("{} ", c)
            }
        }
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

fn get_string_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
