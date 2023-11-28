use std::{env, fs};
use Easy_lang::*;

fn main() {
    let mut lexer = Tokenizer::new(read_file().as_str());
    lexer.create_token();

    println!("{:?}", lexer.token);
}

pub enum Node {
    Expression(Box<Expression>),
    InfixExpression(Box<InfixExpression>),
}

pub struct IntegerLiteral {
    pub value: i64,
}

pub struct Expression {
    pub node: Node,
}

pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Symbol,
}

fn parse_expression(lexer: &mut Tokenizer) -> Expression {}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    if let Some(file_path) = args.get(1) {
        return fs::read_to_string(file_path)
            .expect("Should have been able to read the file")
            .trim()
            .to_string();
    };
    {
        panic!("No file path provided");
    }
}
