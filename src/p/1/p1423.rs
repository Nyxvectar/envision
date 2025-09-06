/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 09/06/2025
 */

use std::io;

fn main() {
    let get = input(1)[0];
    let mut present = 0.0;
    let mut march = 2.0;
    let mut count = 0;
    loop {
        if present >= get {
            break;
        }
        present += march;
        march *= 0.98;
        count += 1;
    }
    print!("{:}", count)
}

fn input(how_many_need: i64) -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}
