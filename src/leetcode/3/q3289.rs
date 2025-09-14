/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn get_sneaky_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut result = vec![];
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                if result.last() != Some(&nums[i]) {
                    result.push(nums[i]);
                }
            }
        }
        result
    }
}
