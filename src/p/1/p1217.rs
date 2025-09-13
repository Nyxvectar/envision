/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let sqrt_n = (n as f64).sqrt() as u32;

    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += 2;
        if i > sqrt_n {
            break;
        }
        if n % i == 0 {
            return false;
        }
        i += 4;
    }

    true
}

fn find_palindrome_primes(a: u32, b: u32) {
    if a <= 5 && 5 <= b {
        println!("5");
    }
    if a <= 7 && 7 <= b {
        println!("7");
    }

    if a <= 11 && 11 <= b {
        println!("11");
    }

    if b >= 100 {
        for d1 in (1..=9).step_by(2) {
            let d1_val = d1 * 100 + d1;
            if d1_val > b {
                break;
            }
            for d2 in 0..=9 {
                let num = d1_val + d2 * 10;
                if num > b {
                    break;
                }
                if num >= a && is_prime(num) {
                    println!("{}", num);
                }
            }
        }
    }

    if b >= 10000 {
        for d1 in (1..=9).step_by(2) {
            let d1_val = d1 * 10000 + d1;
            if d1_val > b {
                break;
            }
            for d2 in 0..=9 {
                let d2_val = d1_val + d2 * 1000 + d2 * 10;
                if d2_val > b {
                    break;
                }
                for d3 in 0..=9 {
                    let num = d2_val + d3 * 100;
                    if num > b {
                        break;
                    }
                    if num >= a && is_prime(num) {
                        println!("{}", num);
                    }
                }
            }
        }
    }

    if b >= 1000000 {
        for d1 in (1..=9).step_by(2) {
            let d1_val = d1 * 1000000 + d1;
            if d1_val > b {
                break;
            }
            for d2 in 0..=9 {
                let d2_val = d1_val + d2 * 100000 + d2 * 10;
                if d2_val > b {
                    break;
                }
                for d3 in 0..=9 {
                    let d3_val = d2_val + d3 * 10000 + d3 * 100;
                    if d3_val > b {
                        break;
                    }
                    for d4 in 0..=9 {
                        let num = d3_val + d4 * 1000;
                        if num > b {
                            continue;
                        }
                        if num >= a && is_prime(num) {
                            println!("{}", num);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let a = parts[0];
    let b = parts[1];

    find_palindrome_primes(a, b);
}
