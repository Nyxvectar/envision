/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/07/2025
 */
struct Solution;

fn main() {
    let result = Solution::xor_operation(5, 0);
    println!("{:?}", result);
}

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut array: Vec<i32> = vec![];
        let mut count = 0;
        for i in 0..n {
            array.push(start + i * 2)
        }
        for k in array {
            count = k ^ count
        }
        count
    }
}
