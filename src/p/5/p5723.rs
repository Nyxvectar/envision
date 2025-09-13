/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let get = input(1)[0];
    let mut num = 2;
    let mut sum = 0;
    let mut count = 0;
    loop {
        if prime(num) {
            if sum + num > get {
                break;
            }
            sum += num;
            count += 1;
            println!("{}", num);
        }
        num += 1;
        if num > get - sum {
            break;
        }
    }
    println!("{}", count)
}

fn prime(num: i64) -> bool {
    let mut temp = true;
    for i in 2..num {
        if num % i == 0 {
            temp = false;
            break;
        }
    }
    temp
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
