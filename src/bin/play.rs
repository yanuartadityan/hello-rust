use std::io;

fn main() {
    let name: Vec<char> = "abcdefg".chars().collect();
    println!("len of string {}", name.len());

    let namaewa: String = String::from("Tomokazu");
    println!("len of string {}", namaewa.len());

    let mut lastname: String = String::from("Nugraha");
    println!("Lastname before change is {lastname}");
    lastname = String::from("Kasatria");
    println!("Lastname after change is {}", lastname);

    // Length of a string
    println!("Length of lastname is {}", lastname.len());

    // Parse input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let parsed_num: u32 = user_input.trim().parse().expect("Failed to parse user input");
    println!("User input: {user_input}");
    println!("Parsed input: {parsed_num}");

    // U-size
    let list_of_words: &str = "women football is bad";
    for word in list_of_words.split_whitespace() {
        let word_len: u16 = word.len() as u16;
        println!("{}'s len is {}", word, word_len);
    }

    // conversion between types
    let _u32: u32 = 123456789;
    let _u16: u16 = 65535;
    let mut u_8: u8 = 255;

    // max range for original type will be used
    println!("u_8: {u_8}");
    u_8 = _u16 as u8;
    println!("u_8 (casted): {u_8}");

    // number literals
    let num_dec: u32 = 12_345;
    println!("decimal: {num_dec}");

    let num_hex: u32 = 0xff;
    println!("hex: {:x}", num_hex);
    println!("HEX: {:X}", num_hex);

    let num_bin: u32 = 0b11101110;
    println!("bin: {:b}", num_bin);
    println!("bin (dec): {}", num_bin);
}