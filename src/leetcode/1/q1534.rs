/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let temp: Vec<i32> = vec![3, 0, 1, 1, 9, 7];
    let result = Solution::count_good_triplets(temp, 7, 2, 3);
    println!("{:?}", result);
}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        let length = arr.len();
        for i in 0..length - 2 {
            for j in i + 1..length - 1 {
                for k in j + 1..length {
                    if (arr[i] - arr[j]).abs() <= a {
                        if (arr[j] - arr[k]).abs() <= b {
                            if (arr[i] - arr[k]).abs() <= c {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}
