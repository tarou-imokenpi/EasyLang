use fast_lang::*;
use std::{collections::binary_heap::Iter, env, fs};

fn main() {
    let mut lexer = Lexer::new(read_file().as_str());
    lexer.create_token();
    println!("{:?}", lexer.token);
    // let mut tokens_iterator = lexer.token.iter();

    // while tokens_iterator.next().is_some() {
    //     let next_token = tokens_iterator.next().unwrap();
    //     todo!()
    // }
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

fn check_next_token(next_token: &Token) {
    todo!()
}
