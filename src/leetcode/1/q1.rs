/**
 * Author:  Raye Lattice
 * Repo:    envision
 * Created: 10/19/2025
 */

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        for i in 0..nums.len() {
            for k in i + 1..nums.len() {
                if nums[i] + nums[k] == target {
                    ans = vec![i as i32, k as i32];
                }
            }
        }
        ans
    }
}