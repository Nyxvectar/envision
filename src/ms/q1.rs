/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/06/2025
 */

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();
        for i in 0..length {
            for k in i..length {
                if i + k == target {
                    return [i, k];
                }
            }
        }
    }
}
