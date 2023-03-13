use std::io;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn main() {
    let user_a: User = build_user(
        {
            println!("Enter user email: ");
            let mut user_email: String = String::new();
            io::stdin()
                .read_line(&mut user_email)
                .expect("Failed to parse user email");
            user_email.trim().to_string()
        },
        {
            println!("Enter user name:" );
            let mut user_name: String = String::new();
            io::stdin()
                .read_line(&mut user_name)
                .expect("Failed to parse user name");
            user_name.trim().to_string()
        }
    );

    println!("{:?}", user_a);
}