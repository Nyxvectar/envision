/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/30/2025
 */

use std::io;

fn main() {
    let input = get_input(2);
    let m = input[0];
    let h = input[1];
    let bmi = m / (h * h);

    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 24.0 {
        println!("Normal");
    } else {
        let rounded = round_to_6_significant(bmi);
        let formatted = format_number(rounded);
        println!("{}", formatted);
        println!("Overweight");
    }
}

fn round_to_6_significant(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }
    let abs_n = n.abs();
    let exp = abs_n.log10().floor() as i32;
    let factor = 10.0f64.powi(5 - exp);
    (n * factor).round() / factor
}

fn format_number(n: f64) -> String {
    let s = n.to_string();
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

fn get_input(how_many_need: i64) -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}
