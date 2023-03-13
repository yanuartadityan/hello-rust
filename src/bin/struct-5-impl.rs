use std::io::{self, Write};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn can_hold(&self, rect_b: &Rectangle) -> bool {
        self.width > rect_b.width && self.height > rect_b.height
    }
}

fn ask_user() -> Rectangle {
    let user_width: u32 = {
        print!("Enter width: ");
        io::stdout().flush().unwrap();
        let mut u_width: String = String::new();
        io::stdin()
            .read_line(&mut u_width)
            .expect("Failed to parse user input width...");
        u_width.trim().parse().expect("Failed to parse")
    };

    let user_height: u32 = {
        print!("Enter height: ");
        io::stdout().flush().unwrap();
        let mut u_height: String = String::new();
        io::stdin()
            .read_line(&mut u_height)
            .expect("Failed to parse user input height...");
        u_height.trim().parse().expect("Failed to parse")
    };

    Rectangle { width: user_width, height: user_height}
}

fn main() {

    let user_box_1: Rectangle = ask_user();
    let user_box_2: Rectangle = ask_user();

    println!("box 2 will fit into box 1: {}", user_box_1.can_hold(&user_box_2));

    // create through square
    let user_box_3: Rectangle = Rectangle::square(20);
    println!("box 3 with size 20 will have area of {}", user_box_3.area());
}