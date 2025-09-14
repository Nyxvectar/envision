/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

use std::collections::HashMap;

struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);
        let mut sum = 0;
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            let present = roman[&s[i]];
            let next = roman[&s[i + 1]];
            if present < next {
                sum -= present;
            } else {
                sum += present;
            }
        }
        sum + roman[&s[s.len() - 1]]
    }
}
