const MINUTE_IN_HOUR: u32 = 60;
const SECOND_IN_MINUTE: u32 = 60;
const SECOND_IN_HOUR: u32 = MINUTE_IN_HOUR * SECOND_IN_MINUTE;

fn main() {
    // global var
    println!("There are {} seconds in an hour", SECOND_IN_HOUR);

    // shadowing
    let firstname: &str = "Andaru";

    {
        let mut firstname: &str = "Satriaka";
        println!("Firstname (shadow) is {}", firstname);

        firstname = "Yotta Nugraha";
        println!("Changed firstname to {}", firstname);
    }

    println!("Original firstname is {}", firstname);

    // String vs &str
    let pangram: &str = "the quick brown fox jumps over the lazy dog";
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // String
    let pangram_str = String::from(pangram);
    println!("Pangram: \n{}", pangram_str);

    // Vector of char
    let pangram_vec: Vec<char> = pangram.chars().collect();
    let mut pangram_str_2 = String::new();
    for word in pangram_vec {
        pangram_str_2.push(word);
    }
    println!("Pangram 2: \n{}", pangram_str_2);
}