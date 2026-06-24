use cli_calculator::evaluate_expression;

fn main() {
    let expression = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    match evaluate_expression(&expression) {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("error: {error:?}");
            std::process::exit(1);
        }
    }
}
