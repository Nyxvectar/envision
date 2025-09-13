/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/13/2025
 */

struct Solution;

fn main() {
    let result = "abcabcabcabc".to_string();
    println!("{:?}", Solution::repeated_substring_pattern(result));
}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s)
    }
}
