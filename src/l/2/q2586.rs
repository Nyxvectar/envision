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
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut count = 0;
        for i in left..=right {
            let temp: Vec<char> = words[i as usize].chars().collect();
            let length = temp.len();
            if vowels.contains(&temp[0]) {
                if vowels.contains(&temp[length - 1]) {
                    count += 1;
                }
            }
        }
        count
    }
}
