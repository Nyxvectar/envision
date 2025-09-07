/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        n > 0 && {
            let mut num = n;
            for factor in [2, 3, 5].iter() {
                while num % factor == 0 {
                    num /= factor;
                }
            }
            num == 1
        }
    }
}
