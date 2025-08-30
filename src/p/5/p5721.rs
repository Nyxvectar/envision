/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/30/2025
 */

use std::io;

fn main() {
    let mut n = get_input(1)[0];
    let mut origin_n = n;
    let mut total_number_display = 0;
    loop {
        total_number_display += n;
        n -= 1;
        if n == 0 {
            break;
        }
    }
    let max_width = {
        let mut k = 0;
        let mut temp = total_number_display;
        loop {
            temp /= 10;
            k += 1;
            if temp == 0 {
                break;
            }
        }
        k
    };
    let mut whiles = 0;
    let keep_n = origin_n;
    if origin_n != 0 {
        if origin_n == 1 {
            print!("01")
        } else {
            for l in 1..=total_number_display {
                print!("{:0width$}", l, width = max_width);
                if l == origin_n {
                    println!();
                    whiles += 1;
                    origin_n += keep_n - whiles;
                }
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
