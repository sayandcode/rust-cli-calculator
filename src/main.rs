use inquire::{CustomType, InquireError, Select};
use std::ops::{Add, Div, Mul, Sub};
use std::process;

trait PromptOption: Sized {
    fn list_variants() -> Vec<Self>;
    fn get_label(&self) -> &str;
}

enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl MathOperation {
    fn calculate<T>(&self, num1: T, num2: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        match self {
            MathOperation::Addition => num1 + num2,
            MathOperation::Subtraction => num1 - num2,
            MathOperation::Multiplication => num1 * num2,
            MathOperation::Division => num1 / num2,
        }
    }
}

impl PromptOption for MathOperation {
    fn list_variants() -> Vec<Self> {
        vec![
            Self::Addition,
            Self::Subtraction,
            Self::Multiplication,
            Self::Division,
        ]
    }

    fn get_label(&self) -> &str {
        match self {
            Self::Addition => "addition ➕",
            Self::Subtraction => "subtraction ➖",
            Self::Multiplication => "multiplication ✖",
            Self::Division => "division ➗",
        }
    }
}

impl Clone for MathOperation {
    fn clone(&self) -> Self {
        match self {
            Self::Addition => Self::Addition,
            Self::Subtraction => Self::Subtraction,
            Self::Multiplication => Self::Multiplication,
            Self::Division => Self::Division,
        }
    }
}

fn main() {
    println!("Welcome to Rust Calculator!");
    let selected_operation = prompt_option::<MathOperation>();
    let first_number = prompt_number("Enter the first number:");
    let second_number = prompt_number("Enter the second number:");
    let result = selected_operation.calculate(first_number, second_number);
    println!("Result: {result}");
}

fn prompt_option<T: PromptOption + Clone>() -> T {
    let option_variants = T::list_variants();
    let prompt_option_labels = option_variants
        .iter()
        .map(|option| option.get_label())
        .collect();
    let selected_option_label = Select::new("Select operation:", prompt_option_labels)
        .prompt()
        .unwrap_or_else(handle_prompt_err);

    let selected_option = option_variants
        .iter()
        .find(|option| option.get_label() == selected_option_label)
        .unwrap_or_else(|| {
            eprintln!("Invalid option: {selected_option_label}");
            process::exit(1);
        });
    return selected_option.clone();
}

fn prompt_number(prompt_label: &str) -> i128 {
    CustomType::<i128>::new(prompt_label)
        .prompt()
        .unwrap_or_else(handle_prompt_err)
}

fn handle_prompt_err<T>(err: InquireError) -> T {
    match err {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
            println!("\nExiting calculator...");
        }
        err => eprintln!("Error: {err}\nExiting calculator..."),
    }
    process::exit(1)
}
