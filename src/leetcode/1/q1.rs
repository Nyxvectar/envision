/**
 * Author:  Raye Lattice
 * Repo:    envision
 * Created: 10/19/2025
 */

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let length = nums.len();
        'out:
        for i in 0..length {
            for k in i + 1..length {
                if nums[i] + nums[k] == target {
                    ans = vec![i as i32, k as i32];
                    break 'out
                }
            }
        }
        ans
    }
}