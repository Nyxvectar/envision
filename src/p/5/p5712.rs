/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let get = get_input(1)[0];
    if get == 0 {
        print!("Today, I ate {} apple.", get)
    } else if get == 1 {
        print!("Today, I ate {} apple.", get)
    } else {
        print!("Today, I ate {} apples.", get)
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
