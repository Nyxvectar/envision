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
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        for i in (0..n).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        digits.insert(0, 1);
        digits
    }

    pub fn plus_one_draft(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        digits[len - 1] += 1;
        for i in 1..len - 1 {
            if digits[len - i] == 10 {
                digits[len - i] = 0;
                digits[len - 1 - i] += 1;
            } else {
                break;
            }
        }
        digits
    }
}
