use std::io;

fn main() {
    let get = input(1)[0];
    let mut temp = 0.0;
    let mut n = 1;
    loop {
        temp += 1.0 / n as f64;
        if temp > get as f64 {
            break;
        } else {
            n += 1;
        }
    }
    print!("{}", n)
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
