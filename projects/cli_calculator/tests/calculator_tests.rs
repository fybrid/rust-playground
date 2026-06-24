use cli_calculator::{evaluate_expression, CalcError};

#[test]
fn evaluates_simple_expression() {
    assert_eq!(evaluate_expression("2 + 3"), Ok(5));
    assert_eq!(evaluate_expression("10 / 2"), Ok(5));
}

#[test]
fn rejects_division_by_zero() {
    assert_eq!(
        evaluate_expression("10 / 0"),
        Err(CalcError::DivisionByZero)
    );
}
