use crate::dirman::echo_yan::echo_yan;

/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 08/13/2025
 */

mod dirman {
    pub mod echo_yan;
}

fn main() {
    let s = String::from(echo_yan());
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}