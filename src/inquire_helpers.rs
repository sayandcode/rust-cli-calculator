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
