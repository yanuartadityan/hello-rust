use std::io;

fn main() {
    let s1: String = gives_ownership();
    let s2: String = String::from("World...");
    let s3: String = gives_and_takes_ownership(s2);

    // valid s1
    println!("s1 {s1} is in scope");
    // invalid s2
    // println!("s2 {s2} is not in scope, hence dropped");
    // valid s3
    println!("s3 {s3} is in scope");

    // string to tuple
    let mut s_usr: String = String::new();
    io::stdin()
        .read_line(&mut s_usr)
        .expect("Failed to parse...");

    let t_usr: (String, usize) = string_to_tuple(s_usr);
    println!("{} is {} chars length", t_usr.0, t_usr.1);
}

fn gives_ownership() -> String {
    let s: String = String::from("Hello...");
    return s;
}

fn gives_and_takes_ownership(s: String) -> String {
    return s;
}

fn string_to_tuple(s: String) -> (String, usize) {
    // let length = s.len();
    (s.trim().to_string(), s.len())
}