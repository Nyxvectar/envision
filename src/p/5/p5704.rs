/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/23/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let long = input.len();
    for k in 1..=long {
        print!("{}", input.chars().nth(long - k).unwrap());
    }
}
