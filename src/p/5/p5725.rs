/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/06/2025
 */
use std::io;

fn main() {
    let n = input(1)[0] as usize;
    print_square(n);
    println!();
    print_triangle(n);
}

fn input(how_many_need: i64) -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}

fn print_square(n: usize) {
    for i in 0..n {
        let mut line = String::new();
        for j in 0..n {
            let num = i * n + j + 1;
            line.push_str(&format!("{:02}", num));
        }
        println!("{}", line);
    }
}

fn print_triangle(n: usize) {
    let mut num = 1;
    for i in 1..=n {
        print!("{:width$}", "", width = 2 * (n - i));
        for _ in 0..i {
            print!("{:02}", num);
            num += 1;
        }
        println!();
    }
}
