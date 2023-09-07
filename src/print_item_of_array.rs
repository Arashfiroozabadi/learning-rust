use std::io;

pub fn print_item_of_array() {
    let array = [1, 2, 3, 4, 5];

    println!("enter an array index");

    let mut array_index = String::new();
    io::stdin()
        .read_line(&mut array_index)
        .expect("failed to read line");

    let array_index: usize = array_index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = array[array_index];

    println!("the value of the element at index {array_index} is: {element}")
}
