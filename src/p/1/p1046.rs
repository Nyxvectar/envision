/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::cmp::max;
use std::io;

fn main() {
    let apples = get_input(10);
    let max_height = get_input(1)[0];
    let mut totally = 0;
    for i in 0..apples.len() {
        if apples[i] <= max_height + 30 {
            totally += 1;
        }
    }
    print!("{}", totally)
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
