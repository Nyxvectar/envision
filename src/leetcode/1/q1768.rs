/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */

struct Solution;

fn main() {
    let para1 = String::from("abc");
    let para2 = String::from("pqr");
    let result = Solution::merge_alternately(para1, para2);
    println!("{:?}", result);
}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        use itertools::Itertools;
        word1.chars().interleave(word2.chars()).collect()
    }
}
