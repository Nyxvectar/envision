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
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.last() {
            Some(word) => word.len() as i32,
            None => 0,
        }
    }
}
