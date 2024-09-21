use calculator::{MathOperation, PromptOption};

mod calculator;
mod inquire_helpers;

fn main() {
    println!("Welcome to Rust Calculator!");
    let selected_operation = MathOperation::prompt();
    let first_number = inquire_helpers::prompt::<f64>("Enter the first number:");
    let second_number = inquire_helpers::prompt::<f64>("Enter the second number:");
    let result = selected_operation.calculate(first_number, second_number);
    println!("Result: {result}");
}
