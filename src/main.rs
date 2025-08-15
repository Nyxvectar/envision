use crate::dirman::echo_yan::echo_yan;

/**
 * Author:  Nyxvectar Yan
 * Repo:    rust-learn
 * Created: 08/15/2025
 */

mod dirman {
    pub mod echo_yan;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<8>,
// }

fn main() {
    let email = "Nyxvectar@proton.me";
    let mut user1 = User {
        email: String::from(email),
        username: String::from(""),
        active: true,
        sign_in_count: 1,
    };

    
    user1.username = echo_yan();
    println!("{}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.active);
    // println!("{:?}", user1);
}

fn build_user(email: String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}