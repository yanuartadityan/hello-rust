use std::io;

fn main() {
    println!("Enter your full name:");
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to parse user input");

    let len = first_space(&s);

    // print
    println!("First word is {} with {} char length", len.1, len.0);
}

// method to find first space
fn first_space(s: &String) -> (usize, &str) {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (i, &s[0..i]);
        }
   } 

   return (s.len(), s);
}