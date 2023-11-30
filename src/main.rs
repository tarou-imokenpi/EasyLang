use std::{env, fs};
use Easy_lang::*;

fn main() {
    let mut tokenizer = Tokenizer::new(read_file().as_str());
    tokenizer.create_token();

    println!("{:?}", &tokenizer.token);

    let expr = binary_expr_parse(&tokenizer.token);
    println!("{:?}", expr);
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

/// 式を表す列挙型
#[derive(Debug)]
enum Expression {
    /// リテラルを表す要素
    Literal(Literal),
    /// 2項演算を表す要素
    BinaryExpr(BinaryExpr),
    /// 関数呼び出しを表す要素関数名と引数を持ちます。
    FunctionCall(String, Vec<Expression>),
}

/// 2項演算子を表す列挙型
#[derive(Debug)]
enum BinOperators {
    /// 加算演算子
    Add,
    /// 減算演算子
    Subtract,
    /// 乗算演算子
    Multiply,
    /// 除算演算子
    Divide,
}

/// リテラルを表す列挙型
#[derive(Debug)]
enum Literal {
    /// 整数を表す要素
    Integer(i64),
    /// 浮動小数点数を表す要素
    Float(f64),
    /// 文字列を表す要素
    String(String),
    /// 真偽値を表す要素
    Boolean(bool),
}

/// 2項演算式を表す構造体
#[derive(Debug)]
struct BinaryExpr {
    /// 左側の式
    left: Box<Expression>,
    /// 右側の式
    right: Box<Expression>,
    /// 演算子
    operator: BinOperators,
}

// let left = Expression::Literal(Literal::Integer(5));
// let right = Expression::Literal(Literal::Integer(3));
// let operator = BinOperators::Add;

// let binary_expr = BinaryExpr {
//     left: Box::new(left),
//     right: Box::new(right),
//     operator,
// };

// let expr = Expression::BinaryExpr(binary_expr);
fn binary_expr_parse(token: &[Token]) -> Expression {
    let mut tokens = token.iter().peekable();
    let mut expr = match tokens.next() {
        Some(Token::Literal(LiteralType::Int(n))) => Expression::Literal(Literal::Integer(*n)),
        Some(Token::Literal(LiteralType::Float(n))) => Expression::Literal(Literal::Float(*n)),
        _ => panic!("Syntax Error - 連続した演算子です。 数値型を期待しています。"),
    };

    while let Some(token) = tokens.next() {
        expr = match token {
            Token::Symbol(Symbol::Plus) => {
                let right = match tokens.next() {
                    Some(Token::Literal(LiteralType::Int(n))) => {
                        Expression::Literal(Literal::Integer(*n))
                    }
                    _ => panic!(
                        "Syntax Error - 連続した演算子です。 次は数値型を期待しています。\n>> {:?}",
                        token
                    ),
                };
                Expression::BinaryExpr(BinaryExpr {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: BinOperators::Add,
                })
            }
            Token::Symbol(Symbol::Minus) => {
                let right = match tokens.next() {
                    Some(Token::Literal(LiteralType::Int(n))) => {
                        Expression::Literal(Literal::Integer(*n))
                    }
                    _ => panic!(
                        "Syntax Error - 連続した演算子です。 次は数値型を期待しています。\n>> {:?}",
                        token
                    ),
                };
                Expression::BinaryExpr(BinaryExpr {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator: BinOperators::Subtract,
                })
            }
            _ => panic!(
                "この演算子はサポートされていません。\nDebug Message: Error Symbol >> {:?}",
                token
            ),
        };
    }

    expr
}
