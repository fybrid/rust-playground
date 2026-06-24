#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    InvalidExpression,
    InvalidNumber(String),
    UnknownOperator(String),
    DivisionByZero,
}

pub fn evaluate_expression(input: &str) -> Result<i64, CalcError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(CalcError::InvalidExpression);
    }

    let left = parse_number(parts[0])?;
    let operator = parse_operator(parts[1])?;
    let right = parse_number(parts[2])?;

    evaluate(left, operator, right)
}

pub fn evaluate(left: i64, operator: Operator, right: i64) -> Result<i64, CalcError> {
    match operator {
        Operator::Add => Ok(left + right),
        Operator::Subtract => Ok(left - right),
        Operator::Multiply => Ok(left * right),
        Operator::Divide if right == 0 => Err(CalcError::DivisionByZero),
        Operator::Divide => Ok(left / right),
    }
}

fn parse_number(input: &str) -> Result<i64, CalcError> {
    input
        .parse()
        .map_err(|_| CalcError::InvalidNumber(input.to_string()))
}

fn parse_operator(input: &str) -> Result<Operator, CalcError> {
    match input {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        other => Err(CalcError::UnknownOperator(other.to_string())),
    }
}
