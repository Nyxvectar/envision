/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let result = Solution::max_score("011101".to_string());
    println!("{:?}", result);
}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let get: Vec<char> = s.chars().collect();
        let mut count = 0;
        for i in 1..s.len() {
            let mut temp = 0;
            for k in 0..i {
                if get[k] == '0' {
                    temp += 1;
                }
            }
            for u in i..s.len() {
                if get[u] == '1' {
                    temp += 1;
                }
            }
            if temp > count {
                count = temp;
            }
        }
        count
    }
}
