use crate::dirman::echo_yan::echo_yan;

/**
 * Author:  Nyxvectar Yan
 * Repo:    rust-learn
 * Created: 08/15/2025
 */

mod dirman {
    pub mod echo_yan;
}

fn main() { 
    let mut i = 0;
    println!("{}", echo_yan());
    for i in 'A'..='z' {
        println!("{}", i)
    }
}
