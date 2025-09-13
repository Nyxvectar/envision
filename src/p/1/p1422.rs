/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::ascii::escape_default;
use std::io;

fn main() {
    let electric_usage = get_input(1)[0];
    let (stage1, stage2, stage3) = (0.4463, 0.4663, 0.5663);
    if electric_usage <= 150 {
        print!("{:.1}", electric_usage as f64 * stage1)
    } else if electric_usage <= 400 {
        print!(
            "{:.1}",
            (electric_usage - 150) as f64 * stage2 + 150.0 * stage1
        )
    } else {
        print!(
            "{:.1}",
            (electric_usage - 400) as f64 * stage3 + 150.0 * stage1 + 250.0 * stage2
        )
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
