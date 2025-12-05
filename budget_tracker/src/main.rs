use std::io;

fn main() {
    println!("Please input a budget: ");

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    let overall_budget: i32 = input1
        .trim()
        .parse()
        .expect("input1 not an integer");

    println!("Overall Budget: {overall_budget}");
}
