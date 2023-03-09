fn main() {
    let dangle_ref = create_dangle_reference();

    // print
    println!("dangle var: {dangle_ref}");
}

fn create_dangle_reference() -> String {
    let s = String::from("Hello");
    s
}