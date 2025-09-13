use std::io;

/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/23/2025
 */

fn main() {
    let mut input = String::new();
    let k = 3;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let input = input.trim();

    for i in 1..=k {
        for _n in 1..=(k - i) {
            print!(" ");
        }
        for _n in 1..=(2 * i - 1) {
            print!("{}", input);
        }
        println!()
    }
}
