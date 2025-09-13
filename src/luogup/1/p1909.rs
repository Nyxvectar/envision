/**
 * Author:  Raye Lattice 
 * Repo:    rustHello
 * Created: 08/30/2025
 */
use std::io;

fn main() {
    let how_many_pencil_need = get_input(1)[0];
    let mut price = vec![];
    let mut package_types = vec![];
    for i in 0..3 {
        package_types.push(get_input(2));
    }
    for k in 0..3 {
        let mut packages_need = 0;
        if how_many_pencil_need % package_types[k][0] == 0 {
            packages_need = how_many_pencil_need / package_types[k][0];
        } else {
            packages_need = how_many_pencil_need / package_types[k][0] + 1;
        }
        price.push(packages_need * package_types[k][1])
    }
    price.sort();
    print!("{}", price[0])
}

fn get_input(how_many_need: i64) -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let get: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    get.into_iter().take(how_many_need as usize).collect()
}
