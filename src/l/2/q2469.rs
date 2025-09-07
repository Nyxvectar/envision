/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/07/2025
 */
struct Solution;

fn main() {
    let para1 = String::from("abc");
    let para2 = String::from("abcd");
    let result = Solution::find_the_difference(para1, para2);
    println!("{:?}", result);
}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        [celsius + 273.15, celsius * 1.80 + 32.00].to_vec()
    }
}
