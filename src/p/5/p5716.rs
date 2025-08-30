/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/30/2025
 */

use std::io;

fn main() {
    let input = get_input(2);
    let mut result = 0;
    if input[0] % 100 == 0 {
        if input[0] % 400 == 0 {
            result = match_month(true, input[1]);
        } else {
            result = match_month(false, input[1]);
        }
    } else {
        if input[0] % 4 == 0 {
            result = match_month(true, input[1]);
        } else {
            result = match_month(false, input[1]);
        }
    }
    print!("{}", result)
}

fn match_month(year: bool, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year == true {
                29
            } else {
                28
            }
        }
        _ => 0,
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
