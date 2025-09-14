/**
 * Author:  Raye Lattice
 * Repo:    rustHello
 * Created: 09/14/2025
 */
use std::i128;

struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn add_binary_simple(a: String, b: String) -> String {
        let ia = i128::from_str_radix(&a, 2).unwrap();
        let ib = i128::from_str_radix(&b, 2).unwrap();
        format!("{:b}", ia + ib).to_string()
    }

    pub fn add_binary_complex(a: String, b: String) -> String {
        let mut temp = Vec::with_capacity(a.len().max(b.len()) + 1);
        temp.push('0');
        for (i, j) in a.chars().rev().zip(b.chars().rev()) {
            let num = i.to_digit(10).unwrap()
                + j.to_digit(10).unwrap()
                + temp.last().and_then(|x| x.to_digit(10)).unwrap();
            *temp.last_mut().unwrap() = char::from_digit(num % 2, 10).unwrap();
            temp.push(char::from_digit(num / 2, 10).unwrap());
        }
        let n = a.len().min(b.len());
        for i in a.chars().rev().skip(n).chain(b.chars().rev().skip(n)) {
            let num = i.to_digit(10).unwrap() + temp.last().and_then(|x| x.to_digit(10)).unwrap();
            *temp.last_mut().unwrap() = char::from_digit(num % 2, 10).unwrap();
            temp.push(char::from_digit(num / 2, 10).unwrap());
        }
        let mut res = temp
            .into_iter()
            .rev()
            .skip_while(|x| *x == '0')
            .collect::<String>();
        if res.is_empty() {
            res.push('0');
        }
        res
    }

    fn binary_to_decimal(binary: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(binary, 2)
    }
}
