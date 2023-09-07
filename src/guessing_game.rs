use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guessing_game() {
  // Guessing Game
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  // println!("The secret number is: {secret_number}");
  loop {
      println!("Input your guess.");

      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => {
            println!("please enter number only");
            continue
          },
      };

      println!("You Guessed: {guess}");

      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
              println!("you win!");
              break;
          }
      }
  }
}