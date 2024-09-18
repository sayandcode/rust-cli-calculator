use inquire::{CustomType, InquireError, Select};
use std::process;

enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn main() {
    println!("Welcome to Rust Calculator!");
    let selected_operation = prompt_operation();
    let first_number = prompt_number("Enter the first number:");
    let second_number = prompt_number("Enter the second number:");
    let result = calculate(first_number, second_number, selected_operation);
    println!("Result: {result}");
}

fn prompt_operation() -> MathOperation {
    let prompt_option_labels = vec![
        "addition ➕",
        "subtraction ➖",
        "multiplication ✖",
        "division ➗",
    ];
    let prompt_result = Select::new("Select operation:", prompt_option_labels).prompt();

    let selected_option_label = unwrap_prompt_result_ok(prompt_result);
    let selected_operation: MathOperation = match selected_option_label {
        "addition ➕" => MathOperation::Addition,
        "subtraction ➖" => MathOperation::Subtraction,
        "multiplication ✖" => MathOperation::Multiplication,
        "division ➗" => MathOperation::Division,
        err => {
            eprintln!("Invalid option: {err}\nExiting calculator...");
            process::exit(1)
        }
    };

    return selected_operation;
}

fn prompt_number(prompt_label: &str) -> i128 {
    let prompt_result = CustomType::<i128>::new(prompt_label).prompt();
    unwrap_prompt_result_ok(prompt_result)
}

fn unwrap_prompt_result_ok<T>(prompt_result: Result<T, InquireError>) -> T {
    match prompt_result {
        Err(err) => {
            match err {
                InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                    println!("\nExiting calculator...");
                }
                err => eprintln!("Error: {err}\nExiting calculator..."),
            }
            process::exit(1)
        }
        Ok(option) => option,
    }
}

fn calculate(num1: i128, num2: i128, operation: MathOperation) -> i128 {
    match operation {
        MathOperation::Addition => num1 + num2,
        MathOperation::Subtraction => num1 - num2,
        MathOperation::Multiplication => num1 * num2,
        MathOperation::Division => num1 / num2,
    }
}

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
