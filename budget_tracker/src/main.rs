use std::{io, process::exit};

fn main() {
    println!("Please input a budget: ");
    let mut parsed_input = 0;

    let mut input_buf = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input_buf) {
        let input = input_buf.replace("$", "");

        parsed_input = match u64::from_str_radix(&input.trim(), 10) {
            Ok(num) => num,
            Err(e) => {
                println!("Your input was invalid, try the 1000 or $1000 format.");
                println!("Error Return: {e}");
                exit(1);
            }
        };
    }

    println!("Overall Budget: {parsed_input}");
}
