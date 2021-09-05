use std::io;

fn main() {
    println!("Enter the equation:");

    let mut eq = String::new();

    io::stdin()
        .read_line(&mut eq)
        .expect("Failed to read the equation");

    let mut operators: Vec<char> = Vec::new();
    for c in eq.chars() {
        match c {
            '+' | '-' | '*' | '/' => operators.push(c),
            _ => {}
        }
    }

    println!("Discovered operators (in reverse order):");
    while let Some(op) = operators.pop() {
        println!("Discovered operator: {}", op);
    }
}
