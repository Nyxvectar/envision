/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let get = input(1)[0];
    let list = input(get);
    let mut count = 1;
    let mut max_count = 0;
    for i in 0..(get - 1) {
        if list[i as usize] + 1 == list[i as usize + 1] {
            count += 1;
        } else {
            count = 1;
        }
        if count > max_count {
            max_count = count;
        }
    }
    print!("{}", max_count)
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
