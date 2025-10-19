/**
 * Author:  Raye Lattice
 * Repo:    envision
 * Created: 10/19/2025
 */

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        strs.sort();
        let first = &strs[0];
        let last = &strs[strs.len() - 1];
        let mut ans = String::new();
        for (a, b) in first.chars().zip(last.chars()) {
            if a != b {
                break;
            }
            ans.push(a);
        }
        ans
    }
}