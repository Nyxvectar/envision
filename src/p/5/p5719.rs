/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let numbers_get = get_input(2);
    let (n, k) = (numbers_get[0], numbers_get[1]);
    let mut atotal = [0, 0];
    let mut btotal = [0, 0];
    for i in 1..=n {
        if i % k == 0 {
            atotal[0] += 1;
            atotal[1] += i;
        } else {
            btotal[0] += 1;
            btotal[1] += i;
        }
    }
    print!(
        "{:.1} {:.1}",
        atotal[1] as f64 / atotal[0] as f64,
        btotal[1] as f64 / btotal[0] as f64
    )
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
