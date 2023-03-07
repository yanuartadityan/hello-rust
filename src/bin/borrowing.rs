fn main() {
    // borrowing
    let s1: String = String::from("Hello");
    let len: usize = calc_str_len(&s1);
    println!("string length is {len}");
    // not valid
    println!("string s1-{s1}");
}

fn calc_str_len(s: &String) -> usize {
    s.len()
}