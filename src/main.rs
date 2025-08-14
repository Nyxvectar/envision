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
    print_name();
    {
        let (a,b) = (1,2);
        let c = a;
        let s1 = String::from("Rust is the best.");
        let s2 = &s1;
        let mut s3 = "Rust defeated Golang.";
        s3 = "Just so so.";
        
        println!("{},{}",a,c);
        println!("{}\n{}",s1,*s2);
        println!("{}", s3);
    }
}

fn print_name() {
    let mut name = "Nyxvectar";
    name = "Nyxvectar Yan";
    println!("{}",echo_yan());
    println!("{}", name);
}
