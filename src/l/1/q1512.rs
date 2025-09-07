/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let temp: Vec<i32> = vec![1, 1, 1, 1];
    let result = Solution::num_identical_pairs(temp);
    println!("{:?}", result);
}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut count = 0;
        for i in 0..length - 1 {
            for k in i + 1..length {
                if nums[i] == nums[k] {
                    count += 1;
                }
            }
        }
        count
    }
}
