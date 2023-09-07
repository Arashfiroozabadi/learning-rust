use std::{thread::sleep, time};

pub fn loops() {
    let items = [10, 20, 30, 40, 50];
    let mut counter = 5;
    let one_sec = time::Duration::from_secs(1);

    // for loop
    for item in items {
        sleep(one_sec);
        println!("item is: {item}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }

    // loop while
    'whileMoz: while counter > 1 {
        sleep(one_sec);
        counter -= 1;
        println!("{counter} time remaining to end while loop");
        if counter <= 0 {
            break 'whileMoz;
        }
    }

    //loop
    'moz: loop {
        sleep(one_sec);
        counter += 1;
        println!("moz {counter}");
        if counter == 5 {
            break 'moz;
        }
    }
}
