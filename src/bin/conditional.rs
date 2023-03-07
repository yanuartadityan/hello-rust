use std::io;
fn main() {
    // simple if-else
    println!("Enter just a random float number:");
    let mut user_txt: String = String::new();
    io::stdin()
        .read_line(&mut user_txt)
        .expect("Failed to parse user input...");
    
    let num: f32 = user_txt.trim().parse().expect("Failed to cast user input...");
    if num < 6.0 {
        println!("the number is smaller...");
    } else {
        println!("the number is bigger...");
    }

    // let expression using conditional if
    let condition = true;
    let user_number = if condition {
        println!("User decide which number...");
        let mut user_txt: String = String::new();
        io::stdin()
            .read_line(&mut user_txt)
            .expect("Failed to parse user input...");

        let num: f32 = user_txt.trim().parse().expect("Failed to cast user input...");
        num
    } else {1000.0};
    println!("the chosen number is {user_number}");
}