use crate::loops::loops;
pub mod loops;

// use crate::guessing_game::guessing_game;
// pub mod guessing_game;

use crate::print_array_item::print_array_items;
pub mod print_array_item;

use crate::print_item_of_array::print_item_of_array;
pub mod print_item_of_array;

fn expression() {
    // its an statement cus its don't return an value
    let a = 8;

    // its an expression cus its evaluate to a value
    let y = {
        let x = 5;
        x + 1
    };

    println!("the value of a is :{a} and its statement cus its not changed");
    println!("the value of y is :{y} and its cus y is equal result of let x + 1 expression");
}

// its an function with return value
fn sum_two_value(a: i32, b: i32) -> i32 {
    let result = a + b;

    if result > 10 {
        println!("value is grater than 10");
    } else {
        println!("value is less than 10");
    }
    result
}

fn main() {
    loops();
    println!("sum two value is: {} ", sum_two_value(3, 6));
    // guessing_game();
    expression();
    print_array_items();
    print_item_of_array()
}
