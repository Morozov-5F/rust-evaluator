use std::io;

fn main() {
    println!("Enter the equation (in RPN):");

    let mut eq = String::new();

    io::stdin()
        .read_line(&mut eq)
        .expect("Failed to read the equation");

    let mut stack: Vec<f64> = Vec::new();

    // TODO: parse the RPN char-by-char
    for word in eq.split_whitespace() {
        match word {
            // If we encountered an operator -- try to evaluate what we have currently on stack
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    println!(
                        "Not enough operands for operator '{}' (got {}, expected 2)",
                        word,
                        stack.len()
                    );
                    break;
                }
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();

                match word {
                    "+" => stack.push(op1 + op2),
                    "-" => stack.push(op1 - op2),
                    "*" => stack.push(op1 * op2),
                    "/" => stack.push(op1 / op2),
                    _ => panic!("Unexpected operator {}", word),
                }
            }
            // Otherwise try to parse the number and error out if parsing fails
            _ => {
                let f = word.parse::<f64>();
                if f.is_err() {
                    println!("Value {} is NaN", word);
                    break;
                }
                let num = f.unwrap();
                stack.push(num);
            }
        }
    }

    if stack.len() != 1 {
        println!("The equation is incomplete, evaluation aborted");
        return;
    }
    println!("Result = {}", stack.pop().unwrap());
}
