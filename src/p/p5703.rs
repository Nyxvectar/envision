/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/23/2025
 */

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (each_get, how_many_people) = (get[0], get[1]);
    print!("{}", each_get * how_many_people)
}
