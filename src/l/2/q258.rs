/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}
