/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let s = |m| n / m * (n / m + 1) / 2 * m;
        s(3) + s(5) + s(7) - s(15) - s(21) - s(35) + s(105)
    }
}
