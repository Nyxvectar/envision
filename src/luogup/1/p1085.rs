/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let mut temp = vec![];
    let mut total = vec![];
    for i in 0..7 {
        temp.push(get_input(2));
        total.push(temp[i][0] + temp[i][1]);
    }
    let mut happy_whole_week = true;
    for k in 0..7 {
        if total[k] > 8 {
            happy_whole_week = false;
        }
    }
    if happy_whole_week == true {
        print!("0")
    } else {
        let mut how = total.clone();
        how.sort();
        for m in 0..7 {
            if total[m] == how[6] {
                print!("{}", m + 1);
                break;
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
