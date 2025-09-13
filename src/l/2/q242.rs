/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();
        s.sort();
        t.sort();
        if s.len() != t.len() {
            return false;
        } else {
            for i in 0..s.len() {
                if s[i] != t[i] {
                    return false;
                }
            }
        }
        true
    }
}
