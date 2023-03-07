fn takes_ownership(string: String) {
    println!("taking {string} now...");
}

fn makes_copy(num: f32) {
    println!("copying {num}");
}

fn main() {
    // ownership & function
    let s1: String = String::from("Hello");
    takes_ownership(s1);
    // not able to use s1 anymore
    // println!("trying to reuse string: {s1}");

    // copying 
    let num: f32 = 3.14;
    makes_copy(num);
    // reusing
    println!("able to reuse num: {num}");
}