use std::io;

fn main() {
    println!(
        "Hello! Welcome to Arithmetic expression
        evaluator."
    );
    println!(
        "You can calculate value for expression such as
        2*3+(4-5)+2^3/4. "
    );
    println!(
        "Allowed numbers: positive, negative and
        decimals."
    );
    println!(
        "Supported operations: Add, Subtract, Multiply,
        Divide, PowerOf(^). "
    );
    println!("Enter your arithmetic expression below:");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(error) => println!("Error in evaluating expressio. Please enter valid expression\n {}", error),
                }
            },
            Err(error) => println!("error: {}", error),
        }
    }
}


fn evaluate(expr: String) -> Result<f64, ParseE> {
    let expr = expr.split_whitespace().collect::<String>();
    
}