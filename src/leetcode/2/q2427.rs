/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).fold(0, |cnt, i| {
            if a % i == 0 && b % i == 0 {
                cnt + 1
            } else {
                cnt
            }
        })
    }
}
