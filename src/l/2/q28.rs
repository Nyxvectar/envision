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
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h: Vec<char> = haystack.chars().collect();
        let n: Vec<char> = needle.chars().collect();
        if needle.len() == 0 {
            return 0;
        } else if needle.len() > haystack.len() {
            return -1;
        } else {
            for i in 0..=h.len() - n.len() {
                let mut match_all = true;
                for k in i..i + needle.len() {
                    if h[k] != n[k - i] {
                        match_all = false;
                        break;
                    }
                }
                if match_all {
                    return i as i32;
                }
            }
        }
        -1
    }
}
