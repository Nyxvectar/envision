/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (s, v) = (get[0], get[1]);
    let mut time_need = (s as f64 / v as f64).ceil() as i32 + 10;

    let mut hours_demand = (time_need as f64 / 60 as f64).ceil() as i32;
    let minutes_demand = time_need % 60;

    let hour = (32 - hours_demand) % 24;
    let minute = (60 - minutes_demand) % 60;

    println!("{:02}:{:02}", hour, minute);
}
