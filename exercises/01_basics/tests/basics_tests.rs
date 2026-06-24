use exercise_01_basics::{
    calculate, celsius_to_fahrenheit, classify_number, fahrenheit_to_celsius,
    format_user_summary, NumberKind, Operation,
};

#[test]
fn converts_temperatures() {
    assert_eq!(celsius_to_fahrenheit(0), 32);
    assert_eq!(celsius_to_fahrenheit(100), 212);
    assert_eq!(fahrenheit_to_celsius(32), 0);
    assert_eq!(fahrenheit_to_celsius(212), 100);
}

#[test]
fn calculates_basic_operations() {
    assert_eq!(calculate(8, 2, Operation::Add), 10);
    assert_eq!(calculate(8, 2, Operation::Subtract), 6);
    assert_eq!(calculate(8, 2, Operation::Multiply), 16);
    assert_eq!(calculate(8, 2, Operation::Divide), 4);
}

#[test]
fn classifies_numbers() {
    assert_eq!(classify_number(-3), NumberKind::Negative);
    assert_eq!(classify_number(0), NumberKind::Zero);
    assert_eq!(classify_number(8), NumberKind::PositiveEven);
    assert_eq!(classify_number(7), NumberKind::PositiveOdd);
}

#[test]
fn formats_user_summary() {
    assert_eq!(
        format_user_summary("Aki", 31, true),
        "Aki is 31 years old and active"
    );
    assert_eq!(
        format_user_summary("Mika", 28, false),
        "Mika is 28 years old and inactive"
    );
}

