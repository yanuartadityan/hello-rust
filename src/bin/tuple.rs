fn main() {
    // init simple tuple
    let tup: (&str, &str, u8) = ("Yanuar", "Nugraha", 35);

    // destructuring a simple tuple
    let (firstname, lastname, age) = tup;

    // accessing simple tuple
    println!("{firstname} {lastname} is {age} years old");
    println!("{} {} is {} years old", tup.0, tup.1, tup.2);

    // mutable tuple
    let mut tup2: (&str, &str, u8) = ("Andaru", "Satriaka", 0);
    tup2.0 = "Nugraha";
    tup2.1 = "Yotta";
    println!("{} {} is {} years old", tup2.0, tup2.1, tup2.2);
}