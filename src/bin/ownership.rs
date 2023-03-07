fn main() {
    // ownership
    let s1: String = String::from("Hello");

    // not possible
    // let s2: s1
    // gotta explicit copy using .clone()
    let _s2 = s1.clone();

    println!("{s1}");

    // works fine with integer/number literals
    let n1: f32 = 3.14;
    let n2 = n1;

    println!("n2: {n2}");
}