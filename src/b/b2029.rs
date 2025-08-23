/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/23/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut get: Vec<f32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (h, r) = (get[0], get[1]);
    let v = (r * r * 3.14 * h) / 1000.0;
    print!("{}", (20.0 / v) as i16 + 1);
}
