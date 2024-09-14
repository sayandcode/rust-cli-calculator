use inquire::{CustomType, Select};
use std::process;

fn main() {
    println!("Welcome to Rust Calculator!");

    // show the user operation options
    let operation_prompt_result = Select::new(
        "Select operation:",
        vec![
            "addition ➕",
            "subtraction ➖",
            "multiplication ✖",
            "division ➗",
        ],
    )
    .prompt();

    let selected_option_label = match operation_prompt_result {
        Err(err) => {
            match err {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    println!("\nExiting calculator...");
                }
                err => eprintln!("Error: {err}\nExiting calculator..."),
            }
            process::exit(1)
        }
        Ok(option) => option,
    };
    enum MathOperation {
        Addition,
        Subtraction,
        Multiplication,
        Division,
    }
    let selected_operation: MathOperation = match selected_option_label {
        "addition ➕" => {
            println!("So you want to add");
            MathOperation::Addition
        }
        "subtraction ➖" => {
            println!("So you want to subtract");
            MathOperation::Subtraction
        }
        "multiplication ✖" => {
            println!("So you want to multiply");
            MathOperation::Multiplication
        }
        "division ➗" => {
            println!("So you want to divide");
            MathOperation::Division
        }
        err => {
            eprintln!("Invalid option: {err}\nExiting calculator...");
            process::exit(1)
        }
    };

    // Get number inputs
    let first_number = match CustomType::<i128>::new("Enter the first number:").prompt() {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
        Ok(num) => num,
    };
    let second_number = match CustomType::<i128>::new("Enter the second number:").prompt() {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
        Ok(num) => num,
    };

    // Calculate
    let result = match selected_operation {
        MathOperation::Addition => first_number + second_number,
        MathOperation::Subtraction => first_number - second_number,
        MathOperation::Multiplication => first_number * second_number,
        MathOperation::Division => first_number / second_number,
    };

    // Show output
    println!("Result: {result}");
}
