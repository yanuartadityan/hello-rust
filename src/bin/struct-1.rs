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
}