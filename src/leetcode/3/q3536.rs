/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut totally = vec![];
        while n != 0 {
            totally.push(n % 10);
            n = n / 10
        }
        totally.sort();
        totally[totally.len() - 1] * totally[totally.len() - 2]
    }
}
