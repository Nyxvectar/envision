/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 08/24/2025
 */
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failure..");
    let input = input.trim();

    let hyphen_positions: Vec<usize> = input
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '-')
        .map(|(i, _)| i)
        .collect();

    let cleaned: String = input.chars().filter(|&c| c != '-').collect();

    if cleaned.len() != 10 {
        println!("fk luogu");
        return;
    }

    let mut digits = Vec::with_capacity(9);
    for c in cleaned.chars().take(9) {
        let d = match c.to_digit(10) {
            Some(num) => num as i32,
            None => {
                println!("why is there not numbers you luogu?");
                return;
            }
        };
        digits.push(d);
    }

    let check_char = cleaned.chars().nth(9).unwrap();
    let check_code = match check_char {
        'X' => 10,
        c => match c.to_digit(10) {
            Some(num) => num as i32,
            None => {
                println!("verify code should be a number or x");
                return;
            }
        },
    };

    let sum: i32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (i + 1) as i32)
        .sum();

    if sum % 11 == check_code % 11 {
        println!("Right");
    } else {
        let correct_check = sum % 11;
        let correct_check_str = if correct_check == 10 {
            "X".to_string()
        } else {
            correct_check.to_string()
        };

        let mut correct_digits = cleaned.chars().take(9).collect::<String>();
        correct_digits.push_str(&correct_check_str);

        let mut correct_isbn = String::new();
        let mut current_pos = 0;

        for &hyphen_pos in &hyphen_positions {
            let digits_to_take = hyphen_pos - current_pos;
            correct_isbn.push_str(&correct_digits[0..digits_to_take]);
            correct_digits = correct_digits[digits_to_take..].to_string();
            correct_isbn.push('-');
            current_pos = hyphen_pos + 1;
        }

        correct_isbn.push_str(&correct_digits);
        println!("{}", correct_isbn);
    }
}
