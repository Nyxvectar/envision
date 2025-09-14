/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for &num in &nums {
            let mut digits = 0;
            let mut n = num;
            while n > 0 {
                n /= 10;
                digits += 1;
            }
            if digits % 2 == 0 {
                count += 1;
            }
        }
        count
    }
}
