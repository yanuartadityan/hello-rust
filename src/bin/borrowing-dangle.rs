fn main() {
    // let dangle_ref = create_dangle_reference();
    let non_dangle_ref = create_non_dangle_reference();
    // print
    println!("dangle var: {non_dangle_ref}");
}

// fn create_dangle_reference() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn create_non_dangle_reference() -> String {
    let s: String = String::from("World");
    s
}