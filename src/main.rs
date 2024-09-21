mod inquire_helpers {
    use inquire::{CustomType, InquireError, Select};
    use std::process;
    use std::str::FromStr;

    trait ErrorHandler<T> {
        fn unwrap_or_handle_err(self) -> T;
    }

    impl<T> ErrorHandler<T> for Result<T, InquireError> {
        fn unwrap_or_handle_err(self) -> T {
            match self {
                Err(err) => {
                    match err {
                        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                            println!("\nExiting calculator...")
                        }
                        err => eprintln!("\nError:{err}/nExiting calculator..."),
                    }
                    process::exit(1)
                }
                Ok(val) => val,
            }
        }
    }

    pub trait PromptOption: Sized + Clone {
        fn list_variants() -> Vec<Self>;

        fn get_label(&self) -> &str;

        fn prompt() -> Self {
            let option_variants = Self::list_variants();
            let prompt_option_labels = option_variants
                .iter()
                .map(|option| option.get_label())
                .collect();
            let selected_option_label = Select::new("Select operation:", prompt_option_labels)
                .prompt()
                .unwrap_or_handle_err();

            let selected_option = option_variants
                .iter()
                .find(|option| option.get_label() == selected_option_label)
                .unwrap_or_else(|| {
                    eprintln!("Invalid option: {selected_option_label}");
                    process::exit(1);
                });
            selected_option.clone()
        }
    }

    pub fn prompt<T: Clone + FromStr + ToString>(label: &str) -> T {
        CustomType::<T>::new(label).prompt().unwrap_or_handle_err()
    }
}

mod calculator {
    pub use crate::inquire_helpers::PromptOption;
    use std::ops::{Add, Div, Mul, Sub};

    pub enum MathOperation {
        Addition,
        Subtraction,
        Multiplication,
        Division,
    }

    impl MathOperation {
        pub fn calculate<T>(&self, num1: T, num2: T) -> T
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
}

use calculator::{MathOperation, PromptOption};

fn main() {
    println!("Welcome to Rust Calculator!");
    let selected_operation = MathOperation::prompt();
    let first_number = inquire_helpers::prompt::<f64>("Enter the first number:");
    let second_number = inquire_helpers::prompt::<f64>("Enter the second number:");
    let result = selected_operation.calculate(first_number, second_number);
    println!("Result: {result}");
}
