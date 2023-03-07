use std::io;
use std::cmp::Ordering;

fn loop_label() {
    // counter
    let mut outer_ctr = 0;
    let mut inner_ctr = 0;

    const OUTER_LIM: u8 = 8;
    const INNER_LIM: u8 = 16;

    // outer
    'outer_loop: loop {
        'inner_loop: loop {
            if inner_ctr == INNER_LIM {
                println!("exiting inner at oidx-{outer_ctr}");
                break 'inner_loop;
            }
            inner_ctr += 1;
        }        

        if outer_ctr == OUTER_LIM {
            println!("exiting outer loop at idx-{outer_ctr}");
            break 'outer_loop;
        }
        outer_ctr += 1;
    }
}

fn while_loop() {
    // while
    let mut number = 4;

    while number > 0 {
        println!("counting down {number}");
        number -= 1;
    }
}

fn looping_collection(reverse: bool) {
    // ask user max range
    let mut user_max: String = String::new();
    io::stdin()
        .read_line(&mut user_max)
        .expect("Failed to parse user input...");

    let n_user_max: i32 = user_max.trim().parse().expect("Failed to cast user input...");

    // create range
    let list_num: Vec<i32> = if reverse {
        (0..n_user_max).rev().collect()
    } else {
        (0..n_user_max).collect()
    };

    // loop
    for el in list_num {
        print!("{el} --> ");
        match el.cmp(&50) {
            Ordering::Equal => println!("Great!"),
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Greater!")
        }
    }
}

fn main() {
    // loop
    let mut counter = 0;
    let result = loop {
        if counter == 100 {
            break counter * 2;
        }
        counter += 1;
    };
    
    println!("counter after break is {counter} with result {result}");

    // nested loop with label
    loop_label();

    // while loop
    while_loop();

    // looping a collection
    looping_collection(false);
}