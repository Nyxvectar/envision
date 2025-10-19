/**
 * Author:  Raye Lattice
 * Repo:    envision
 * Created: 10/19/2025
 */

use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut res = 0;

        while i < j {
            let min_h = cmp::min(height[i], height[j]);
            let area = ((j - i) as i32) * min_h;
            res = cmp::max(res, area);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        res
    }
}