use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<f32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let long = input.len();
    for k in 1..=long {
        print!("{}", input.chars().nth(long - k).unwrap());
    }
}
