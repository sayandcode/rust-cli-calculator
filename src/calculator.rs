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
