/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let input = get_input(2);
    let x = input[0];
    let n = input[1] as usize;
    let mut total = vec![0];
    let mut current_day = x;
    for _ in 0..n {
        if current_day != 6 && current_day != 7 {
            add(&mut total, 250);
        }
        current_day = if current_day == 7 { 1 } else { current_day + 1 };
    }

    for digit in total.iter().rev() {
        print!("{}", digit);
    }
    println!();
}

fn add(num: &mut Vec<u32>, value: u32) {
    let mut carry = value;
    let mut i = 0;

    while carry > 0 || i < num.len() {
        let sum = carry + if i < num.len() { num[i] } else { 0 };
        if i < num.len() {
            num[i] = sum % 10;
        } else {
            num.push(sum % 10);
        }
        carry = sum / 10;
        i += 1;
    }
}

fn get_input(how_many: usize) -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .take(how_many)
        .collect()
}
