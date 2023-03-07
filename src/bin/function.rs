use std::io;

const PI: f32 = 3.14;

fn calc_area(r: f32) -> f32 {
   PI * r.powf(2 as f32) /* no need to type return explicitly */
}

fn calc_circumference(r: f32) -> f32 {
    return 2 as f32 * PI * r; /* old way of return statement */ 
}

fn main(){
    // get user input on radius
    println!("Enter radius: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input...");
    let radius: f32 = input.trim().parse().expect("Failed to parse...");

    // get area and circumference
    let area: f32 = calc_area(radius);
    let circ: f32 = calc_circumference(radius);
    println!("with radius {:.3} area is {:.3} and circ is {:.3}", radius, area, circ);
}