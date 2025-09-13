/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let input = input.trim();
    let (sign, num_str) = if input.starts_with('-') {
        ("-", &input[1..])
    } else {
        ("", input)
    };
    let reversed: String = num_str.chars().rev().collect();
    let reversed_trimmed = reversed.trim_start_matches('0');
    let result = if reversed_trimmed.is_empty() {
        "0".to_string()
    } else {
        format!("{}{}", sign, reversed_trimmed)
    };
    println!("{}", result);
}
