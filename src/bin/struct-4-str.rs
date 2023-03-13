use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\n\tusername: {}, \n\temail: {}, \n\tstatus: {}\n]", 
            self.username, self.email, self.active)
    }
} 

fn main() {
    let user_a = User {
        active: true,
        username: String::from("mirkwood"),
        email: String::from("mirkwood@gege.com")
    };

    println!("{}", user_a)
}