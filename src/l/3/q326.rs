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
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && (3 as i32).pow(19) % n == 0
    }
}
