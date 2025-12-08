use std::{io, process::exit};
use std::collections::HashMap;

fn main() {
    print!("Please input a budget: "); // input prompt
    let mut parsed_input = 0;

    let mut input_buf = String::new();
    println!("");

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

    print!("How many items are you paying for with this budget?: "); // input prompt
    let mut item_count = 0;

    input_buf = String::new(); // already declared this variable, just shadowing it
    println!("");
    if let Ok(_) = io::stdin().read_line(&mut input_buf) {
        item_count = match u64::from_str_radix(&&input_buf.trim(), 10) {
            Ok(num) => num,
            Err(e) => {
                println!("Your input was invalid, try a smaller positive number.");
                println!("Error Return: {e}");
                exit(1);
            }
        };
    }

    let mut item_list: HashMap<String, String> = HashMap::new();
    let mut all_costs: Vec<u64> = vec![];

    for i in 1..=item_count {
        print!("What is the name of item number {i}?: ");
        //problem: runs read_line before print
        let mut item = String::new(); // creates a string variable to store information about the cost
        io::stdin().read_line(&mut item).expect("Failed to read line");
        println!("");

        print!("Cost of item number {i}: ");
        let mut item_cost: u64 = 0;

        let mut cost_buf = String::new();
        println!("");

        if let Ok(_) = io::stdin().read_line(&mut cost_buf) {
            item_cost = match u64::from_str_radix(&&&cost_buf.trim(), 10) {
                Ok(num) => num,
                Err(e) => {
                    println!("Your input was invalid, try a smaller positive number.");
                    println!("Error Return: {e}");
                    exit(1);
                }
            };
        }

        println!("Item: {item}; Item Cost: {cost_buf} ");
        item_list.insert(
            item,
            cost_buf,
        );

        all_costs.push(item_cost);
    }

    for (k, v) in &item_list {
        println!("{k}: {v}");
    }

    let mut total_cost: u64 = 0;

    for j in 0..all_costs.len() {
        total_cost += all_costs[j];
    }

    let remaining_budget: u64 = parsed_input - total_cost;

    println!("Total Cost: {total_cost}");
    println!("Remaining Budget {remaining_budget}");
}
