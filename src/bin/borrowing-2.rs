fn main() {
    // borrowing
    let mut s1: String = String::from("Hello");

    // borrow
    let r1 = &s1;
    let r2 = &s1;

    // use r1 and r2
    println!("first use: r1-{r1}, r2-{r2}");

    // reuse r1 and r2
    println!("second use: r1-{r1}, r2-{r2}");

    // reborrow now
    let r3 = &mut s1;
    change(r3);
    println!("reborrow: r3-{r3}");

    // check if r1 and r2 can be used that s1 has changed
    // short answer: error
    // println!("third use: r1-{r1}, r2-{r2}");
}

fn change(s: &mut String) {
    s.push_str(" World");
}