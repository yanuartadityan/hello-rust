use std::fmt;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: ({}, {}, {})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: ({}, {}, {})", self.0, self.1, self.2)
    }
}
fn main() {
    let black: Color = Color(0, 0, 0);
    let corner: Point = Point(10, 10, 10);

    println!("{}", black);
    println!("{}", corner);
}