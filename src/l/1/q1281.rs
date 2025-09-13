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
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut product = 1;
        let mut sum = 0;
        for i in to_digits(n) {
            product *= i
        }
        for k in to_digits(n) {
            sum += k
        }
        product - sum
    }
}

fn to_digits(mut num: i32) -> Vec<i32> {
    if num == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    num = num.abs();
    while num > 0 {
        let last_digit = num % 10;
        digits.push(last_digit);
        num /= 10;
    }
    digits.reverse();
    digits
}
