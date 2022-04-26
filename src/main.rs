use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let reset = "\u{001b}[0m";
    let cyan = "\u{001b}[36m";
    let magenta = "\u{001b}[35;1m";
    let red = "\u{001b}[31;1m";
    let yellow = "\u{001b}[33;1m";
    let green = "\u{001b}[32;1m";

    println!("{cyan}Guess the number!\n", cyan=cyan);

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}{reset}", secret_number, reset=reset);

    loop {
        println!("\n{magenta}Please input your guess.", magenta=magenta);

        let mut guess = String::new();

        println!("{}", reset);

        io::stdin()
            .read_line(&mut guess) // Returns an enum io::Result
            .expect("Failed to read line");
            // Result has two variants, Ok and Err, if its Err, expect will crash with that message

        // Rust allows change the data type for variable reusing
        // u32 is unsigned 32-bit integer.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("\nYou guessed: {}{reset}\n", guess, reset=reset);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{red}Too small!{reset}", red=red, reset=reset),
            Ordering::Greater => println!("{yellow}Too big!{reset}", yellow=yellow, reset=reset),
            Ordering::Equal => {
                println!("{green}You win!{reset}", green=green, reset=reset);
                break;
            }
        }
    }
}
