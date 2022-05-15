use std::io::{self, Write};

enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
}

impl Operator {
    pub fn operate(&self, a: f64, b: f64) -> f64 {
        match *self {
            Operator::Addition => a + b,
            Operator::Subtraction => a - b,
            Operator::Multiplication => a * b,
            Operator::Division => a / b,
            Operator::Exponent => a.powf(b),
        }
    }
}

enum Token {
    Number(f64),
    Operator(Operator),
}

fn evaluate_problem(problem: Vec<Token>) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for character in problem {
        match character {
            Token::Number(num) => stack.push(num as f64),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return None;
                }
                let last_index = stack.len() - 1;
                stack[last_index - 1] = op.operate(stack[last_index - 1], stack[last_index]);
                stack.pop();
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    Some(stack[0])
}

fn string_to_equation(string: String) -> Option<Vec<Token>> {
    let tokens: Vec<&str> = string.split(' ').collect();

    let mut equation = Vec::new();
    for token in tokens {
        equation.push(match token.parse::<f64>() {
            Ok(num) => Token::Number(num),
            Err(_) => match token {
                "+" => Token::Operator(Operator::Addition),
                "-" => Token::Operator(Operator::Subtraction),
                "*" => Token::Operator(Operator::Multiplication),
                "/" => Token::Operator(Operator::Division),
                "^" => Token::Operator(Operator::Exponent),
                _ => {
                    return None;
                }
            },
        });
    }
    Some(equation)
}

fn main() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_owned();

        if &input[..].to_ascii_lowercase() == "exit" || &input[..].to_ascii_lowercase() == "quit" {
            break;
        }

        match string_to_equation(input.clone()) {
            Some(problem) => match evaluate_problem(problem) {
                Some(val) => println!("{}", val),
                None => println!("Invalid equation"),
            },
            None => {
                println!("Invalid token in equation");
            }
        }
    }
}
