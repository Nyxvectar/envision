/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || x > 0 && x % 10 == 0 {
            return false;
        }
        let mut rev = 0;
        while rev < x / 10 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        rev == x || rev == x / 10
    }
}
