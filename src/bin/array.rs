use std::io;
fn main() {
    // simple array
    let simp_arr = [1, 2, 3, 4, 5];
    println!("array {:?}", simp_arr);

    // simple array with type
    let simp_arr: [u8; 5] = [1, 4, 9, 16, 25];
    println!("array {:?}", simp_arr);

    // accessing array
    println!("which index to read:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read user input...");

    let index_num: usize = index.trim().parse().expect("Failed to parse...");
    println!("array at index-{} is {}", index_num, simp_arr[index_num]);
}