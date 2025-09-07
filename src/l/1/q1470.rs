/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

use itertools::Itertools;

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let length = n as usize;
        let mut result = Vec::with_capacity(2 * length);
        for i in 0..n {
            result.push(nums[i as usize]);
            result.push(nums[i as usize + length])
        }
        result
    }
}
