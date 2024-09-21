use calculator::MathOperation;
use inquire_helpers::PromptOption;

mod calculator;
mod inquire_helpers;

fn main() {
    loop {
        println!("{esc}c", esc = 27 as char); // clear screen
        println!("Welcome to Rust Calculator!");

        let selected_operation = MathOperation::prompt();
        let first_number = inquire_helpers::prompt_input::<f64>("Enter the first number:");
        let second_number = inquire_helpers::prompt_input::<f64>("Enter the second number:");
        let result = selected_operation.calculate(first_number, second_number);
        println!("Result: {result}");

        let is_repeat = inquire_helpers::prompt_confirm("Another round?");
        if !is_repeat {
            break;
        }
    }
}
