/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|x| format!("{:b}", x.parse::<u16>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}