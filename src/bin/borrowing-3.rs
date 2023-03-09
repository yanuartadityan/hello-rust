fn main() {
    // create a mutable string (stack), with "Netflix" is
    // located in heap
    let mut s1: String = String::from("Netflix");

    // within this scope try to borrow s1 in r1
    {
        // borrowing
        let r1 = &mut s1;

        // while borrowing r1, pass it to function
        change(r1);

        // the heap is now updated
        println!("r1-{r1}");
    }

    // re-borrow s1 outside the scope, valid
    let r2 = &mut s1;

    // update the heap
    change(r2);
    
    // the new heap is updated
    println!("r2-{r2}");
}

fn change(s: &mut String) {
    s.push_str(" Sucks!");
}