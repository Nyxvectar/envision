/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let get = get_input(3);
    let (m, t, s) = (get[0], get[1], get[2]);
    if t == 0 {
        print!("0")
    } else {
        if (s / t) >= m {
            print!("0")
        } else {
            if s % t == 0 {
                print!("{}", m - s / t)
            } else {
                print!("{}", m - 1 - s / t)
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
