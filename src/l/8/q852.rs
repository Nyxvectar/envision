/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/07/2025
 */
use itertools::Itertools;

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut temp = arr.clone();
        temp.sort();
        let mut count = 0;
        for i in 0.. {
            if arr[i] == temp[temp.len() - 1] {
                count = i;
                break;
            }
        }
        count as i32
    }
}
