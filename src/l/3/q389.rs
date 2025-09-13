/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */

struct Solution;

fn main() {
    let para1 = String::from("abc");
    let para2 = String::from("abcd");
    let result = Solution::find_the_difference(para1, para2);
    println!("{:?}", result);
}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s_vector: Vec<char> = s.chars().collect();
        let mut t_vector: Vec<char> = t.chars().collect();
        s_vector.sort();
        t_vector.sort();
        for i in 0..s_vector.len() {
            if s_vector[i] != t_vector[i] {
                return t_vector[i];
            }
        }
        t_vector[t_vector.len() - 1]
    }
}
