/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|x| {(x[1]as i32 - x[0] as i32).abs()})
            .sum()
    }
}