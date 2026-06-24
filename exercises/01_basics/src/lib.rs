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
    todo!("convert Celsius to Fahrenheit")
}

pub fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    todo!("convert Fahrenheit to Celsius")
}

pub fn calculate(left: i32, right: i32, operation: Operation) -> i32 {
    todo!("implement the calculator with match")
}

pub fn classify_number(value: i32) -> NumberKind {
    todo!("classify negative, zero, positive even, and positive odd values")
}

pub fn format_user_summary(name: &str, age: u8, active: bool) -> String {
    todo!("return a readable user summary")
}

