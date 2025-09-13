/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let get = get_input(1);
    let result = [{ get[0] % 2 == 0 }, get[0] > 4 && get[0] <= 12];
    if result[0] == true && result[1] == true {
        print!("1 ")
    } else {
        print!("0 ")
    }
    if result[0] == true || result[1] == true {
        print!("1 ")
    } else {
        print!("0 ")
    }
    if result[0] == true && result[1] == false {
        print!("1 ")
    } else if result[1] == true && result[0] == false {
        print!("1 ")
    } else {
        print!("0 ")
    }
    if result[0] == false && result[1] == false {
        print!("1 ")
    } else {
        print!("0 ")
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
