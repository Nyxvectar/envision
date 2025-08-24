/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/24/2025
 */

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let result = get[0];
    let mut ans = 0;
    for i in 1..=result {
        ans += calculate(i)
    }
    print!("{}", ans)
}

fn calculate(num: i64) -> i64 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}
