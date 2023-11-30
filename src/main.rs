use std::{env, fs};
use Easy_lang::*;

fn main() {
    let mut tokenizer = Tokenizer::new(read_file().as_str());
    tokenizer.create_token();
    println!("{:?}", &tokenizer.token);

    // let expr = binary_expr_parse(&tokenizer.token);
    // println!("{:?}", expr);

    // match expr {
    //     BinaryExpr => {
    //         println!("{:?}", Expression::BinaryExpr(left));
    //         println!("{:?}", right);
    //         println!("{:?}", operator);
    //     }
    // }
}

/// ファイルを読み込みテキストを返します
fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_path) => fs::read_to_string(file_path)
            .expect("ファイルが読めるはずでした。")
            .trim()
            .to_string(),

        None => panic!("ファイルパスが指定されていません"),
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

fn binary_expr_parse(token: &[Token]) -> Expression {
    let mut tokens = token.iter().peekable();
    let mut expr = match tokens.next() {
        Some(Token::Literal(LiteralType::Int(n))) => Expression::Literal(Literal::Integer(*n)),
        Some(Token::Literal(LiteralType::Float(n))) => Expression::Literal(Literal::Float(*n)),
        _ => panic!("Syntax Error - 連続した演算子です。 数値型を期待しています。"),
    };

    while let Some(token) = tokens.next() {
        let ope = match token {
            Token::Symbol(Symbol::Plus) => BinOperators::Add,
            Token::Symbol(Symbol::Minus) => BinOperators::Subtract,
            Token::Symbol(Symbol::Asterisk) => BinOperators::Multiply,
            Token::Symbol(Symbol::Slash) => BinOperators::Divide,
            _ => panic!("未対応の演算子です。"),
        };
        expr = {
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
                operator: ope,
            })
        };
    }

    expr
}
