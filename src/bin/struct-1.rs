use std::io::{self, Read};

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let a: User = User {
        active: true,
        username: String::from("mirkwood.ape"),
        email: String::from("mirkwoodape@gmail.com"),
        sign_in_count: 1
    };

    let b = User {
        active: false,
        username: {
            let mut asu: String = String::new();
            io::stdin()
                .read_line(&mut asu)
                .expect("Failed to parse username..");
            asu.trim().to_string()
        },
        ..a
    };

    println!("b: {:?}", b);
}