use std::{env, fs, slice::Iter};
use Easy_lang::*;

fn main() {
    let mut tokenizer = Tokenizer::new(read_file().as_str());
    tokenizer.create_token();

    let mut token_iterator: Iter<'_, Token> = tokenizer.token.iter();
    let parsed_express = parse_expression(&mut token_iterator);
    println!("{:?}", tokenizer.token);
    println!("{:?}", parsed_express);
}

#[derive(Debug)]
pub enum Node {
    Expression(Box<Expression>),
    InfixExpression(Box<InfixExpression>),
    IntegerLiteral(Box<NumberLiteral>),
}
#[derive(Debug)]
pub struct NumberLiteral {
    pub value: i64,
}

#[derive(Debug)]
pub struct Expression {
    pub node: Node,
}

// 中置演算子の構造体
#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Symbol,
}

// iteratorを使う
fn parse_expression(token_iter: &mut Iter<'_, Token>) -> Expression {
    // どう繰り返すか
    match token_iter.next() {
        Some(token) => match token {
            Token::Number(number) => Expression {
                node: Node::IntegerLiteral(Box::new(NumberLiteral {
                    value: match number {
                        NumberType::Integer(value) => *value,
                        NumberType::Float(value) => *value as i64,
                    },
                })),
            },
            Token::Symbol(symbol) => {
                let left = parse_expression(token_iter);
                let right = parse_expression(token_iter);
                Expression {
                    node: Node::InfixExpression(Box::new(InfixExpression {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator: symbol.clone(),
                    })),
                }
            }
            Token::Reserved(_) => todo!(),
            Token::String(_) => todo!(),
            Token::Identifier(_) => todo!(),
        },
        None => todo!(),
    }
}

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
