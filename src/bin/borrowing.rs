fn main() {
    // borrowing
    let s1: String = String::from("Hello");
    let len: usize = calc_str_len(&s1);
    println!("string length is {len}");
    // still valid, due to borrowing
    println!("1. string s1-{s1}");

    // passing a reference (mutable reference)
    let mut s2: String = String::from("World");
    change(&mut s2);
    println!("2. string s2-{s2}");

    // mutable reference (more)
    let mut s3: String = String::from("Jeremy Clarkson");
    let r1 = &mut s3;
    // let r2 = &mut s3; /* can't borrow variable that is already borrowed */
    // below, r1 is being used, and that ends the borrowing time
    println!("3. r1-{r1}");
    // now r2 can borrow s3 as well
    let r2 = &mut s3;
    println!("4. r2-{r2}");
}

fn calc_str_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" Cruel");
}