/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/13/2025
 */

struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|x| *x != 0);
        nums.resize(len, 0);
    }
}
