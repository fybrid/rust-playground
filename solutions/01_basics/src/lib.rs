#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    Negative,
    Zero,
    PositiveEven,
    PositiveOdd,
}

pub fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

pub fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

pub fn calculate(left: i32, right: i32, operation: Operation) -> i32 {
    match operation {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide => left / right,
    }
}

pub fn classify_number(value: i32) -> NumberKind {
    match value {
        value if value < 0 => NumberKind::Negative,
        0 => NumberKind::Zero,
        value if value % 2 == 0 => NumberKind::PositiveEven,
        _ => NumberKind::PositiveOdd,
    }
}

pub fn format_user_summary(name: &str, age: u8, active: bool) -> String {
    let status = if active { "active" } else { "inactive" };
    format!("{name} is {age} years old and {status}")
}

