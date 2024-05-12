use std::io;

#[derive(Debug, Clone)]
enum Node {
    Literal(i32),
    BinOp(Operation, Box<Node>, Box<Node>),
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Token {
    Number(i32),
    Operator(Operation),
    LeftParen,
    RightParen,
}

fn tokenise(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        if c.is_digit(10) {
            let mut value = c.to_digit(10).unwrap() as i32;
            iter.next();
            while let Some(&c) = iter.peek() {
                if c.is_digit(10) {
                    value = value * 10 + c.to_digit(10).unwrap() as i32;
                    iter.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(value));
        } else {
            match c {
                '+' => {
                    tokens.push(Token::Operator(Operation::Add));
                    iter.next();
                }
                '-' => {
                    tokens.push(Token::Operator(Operation::Subtract));
                    iter.next();
                }
                '*' => {
                    tokens.push(Token::Operator(Operation::Multiply));
                    iter.next();
                }
                '/' => {
                    tokens.push(Token::Operator(Operation::Divide));
                    iter.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    iter.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    iter.next();
                }
                _ if c.is_whitespace() => {
                    iter.next();
                }
                _ => panic!("Invalid token: {}", c),
            }
        }
    }

    tokens
}

fn parse_expression(tokens: &[Token], index: usize) -> (Node, usize) {
    let (mut lhs, mut next_index) = parse_term(tokens, index);

    while next_index < tokens.len() {
        match tokens[next_index] {
            Token::Operator(op) => {
                let (rhs, next_next_index) = parse_term(tokens, next_index + 1);
                lhs = Node::BinOp(op, Box::new(lhs), Box::new(rhs));
                next_index = next_next_index;
            }
            Token::RightParen => break,
            _ => panic!("Unexpected token: {:?}", tokens[next_index]),
        }
    }

    (lhs, next_index)
}

fn parse_term(tokens: &[Token], index: usize) -> (Node, usize) {
    let (mut lhs, mut next_index) = parse_factor(tokens, index);

    while next_index < tokens.len() {
        match tokens[next_index] {
            Token::Operator(Operation::Multiply) | Token::Operator(Operation::Divide) => {
                let op = match tokens[next_index] {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                };

                let (rhs, next_next_index) = parse_factor(tokens, next_index + 1);
                lhs = Node::BinOp(op, Box::new(lhs), Box::new(rhs));
                next_index = next_next_index;
            }
            _ => break,
        }
    }

    (lhs, next_index)
}

fn parse_factor(tokens: &[Token], index: usize) -> (Node, usize) {
    match tokens[index] {
        Token::Number(value) => (Node::Literal(value), index + 1),
        Token::LeftParen => {
            let (expr, next_index) = parse_expression(tokens, index + 1);
            if tokens[next_index] == Token::RightParen {
                (expr, next_index + 1)
            } else {
                panic!("Expected ')'");
            }
        }
        _ => panic!("Unexpected token: {:?}", tokens[index]),
    }
}

fn build_ast(input: &str) -> Node {
    let tokens = tokenise(input);
    let (ast, _) = parse_expression(&tokens, 0);
    ast
}

fn visualise_ast(node: &Node, level: usize) {
    match node {
        Node::Literal(value) => println!("{}- Literal({})", "|   ".repeat(level), value),
        Node::BinOp(op, lhs, rhs) => {
            println!("{}- BinOp({:?})", "|   ".repeat(level), op);
            visualise_ast(lhs, level + 1);
            visualise_ast(rhs, level + 1);
        }
    }
}

fn main() {
    println!("Enter an expression to parse:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove trailing newline
    let ast = build_ast(input);
    visualise_ast(&ast, 0);
}
