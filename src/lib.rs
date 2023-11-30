#[derive(Debug)]
pub enum Token {
    Reserved(ReservedWord),
    Symbol(Symbol),
    Literal(LiteralType),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Dot,
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    And,
    Or,
    Not,
    OriginalName,
}

#[derive(Debug)]
pub enum ReservedWord {
    If,
    Else,
    While,
    For,
    In,
    Function,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
    Var,
    Let,
    Const,
    Print,
    Println,
    Input,
    Len,
    Type,
    Import,
    From,
    As,
    Class,
    New,
    This,
    Super,
}

#[derive(Debug)]
pub enum LiteralType {
    Int(i64),
    Float(f64),
    String(String),
}

pub trait TokenizerTrait {
    fn new(code: &str) -> Self;
    fn next(&mut self);
    fn create_number(&mut self);
    fn create_token(&mut self);
}

pub struct Tokenizer {
    pub text: Vec<char>,
    current_index: usize,
    current_char: char,
    max_index: usize,
    pub token: Vec<Token>,
}

impl TokenizerTrait for Tokenizer {
    fn new(code: &str) -> Self {
        let text: Vec<char> = code.chars().collect();
        let current_index = 0;
        let max_index = text.len() - 1;
        let token: Vec<Token> = Vec::new();
        let current_char = text[current_index];
        Self {
            text,
            current_index,
            current_char,
            max_index,
            token,
        }
    }

    fn next(&mut self) {
        self.current_index += 1;
        if self.current_index > self.max_index {
            self.current_char = '\0';
        } else {
            self.current_char = self.text[self.current_index];
        }
    }

    fn create_number(&mut self) {
        let mut number = String::new();
        while self.current_char.is_digit(10) {
            number.push(self.current_char);
            self.next();
        }
        if self.current_char == '.' {
            number.push(self.current_char);
            self.next();
            while self.current_char.is_digit(10) {
                number.push(self.current_char);
                self.next();
            }
            self.token
                .push(Token::Literal(LiteralType::Float(number.parse().unwrap())));
        } else {
            self.token
                .push(Token::Literal(LiteralType::Int(number.parse().unwrap())));
        }
    }

    // トークンを作成する
    // 1. 空白文字をスキップする
    // 2. 数字を作成する
    // 3. 記号を作成する
    // 4. 予約語を特定する、そうでなければ、文字列として作成する
    fn create_token(&mut self) {
        while self.current_char != '\0' {
            // 空白文字をスキップ
            if self.current_char.is_whitespace() {
                self.next();
                continue;
            }
            // 数字
            if self.current_char.is_digit(10) {
                self.create_number();
                continue;
            }
            // 記号
            if let Some(symbol) = match self.current_char {
                '+' => Some(Symbol::Plus),
                '-' => Some(Symbol::Minus),
                '*' => Some(Symbol::Asterisk),
                '/' => Some(Symbol::Slash),
                '%' => Some(Symbol::Percent),
                '=' => Some(Symbol::Equal),
                '!' => Some(Symbol::Not),
                '<' => Some(Symbol::LessThan),
                '>' => Some(Symbol::GreaterThan),
                '&' => Some(Symbol::And),
                '|' => Some(Symbol::Or),
                '(' => Some(Symbol::LeftParen),
                ')' => Some(Symbol::RightParen),
                '[' => Some(Symbol::LeftBracket),
                ']' => Some(Symbol::RightBracket),
                '{' => Some(Symbol::LeftBrace),
                '}' => Some(Symbol::RightBrace),
                ',' => Some(Symbol::Comma),
                ':' => Some(Symbol::Colon),
                ';' => Some(Symbol::Semicolon),
                '.' => Some(Symbol::Dot),
                '"' => {
                    self.next();
                    let mut string = String::new();
                    while self.current_char != '"' {
                        string.push(self.current_char);
                        self.next();
                    }
                    self.token.push(Token::Literal(LiteralType::String(string)));
                    self.next();
                    continue;
                }
                _ => None,
            } {
                self.token.push(Token::Symbol(symbol));
                self.next();
                continue;
            }
            // アルファベット
            if self.current_char.is_alphabetic() {
                let mut identifier = String::new();
                while self.current_char.is_alphabetic() {
                    identifier.push(self.current_char);
                    self.next();
                }
                // 予約語に一致するかどうか
                if let Some(reserved) = match identifier.as_str() {
                    "if" => Some(ReservedWord::If),
                    "else" => Some(ReservedWord::Else),
                    "while" => Some(ReservedWord::While),
                    "for" => Some(ReservedWord::For),
                    "in" => Some(ReservedWord::In),
                    "fn" => Some(ReservedWord::Function),
                    "ret" => Some(ReservedWord::Return),
                    "break" => Some(ReservedWord::Break),
                    "continue" => Some(ReservedWord::Continue),
                    "true" => Some(ReservedWord::True),
                    "false" => Some(ReservedWord::False),
                    "null" => Some(ReservedWord::Null),
                    "var" => Some(ReservedWord::Var),
                    "let" => Some(ReservedWord::Let),
                    "const" => Some(ReservedWord::Const),
                    "p" => Some(ReservedWord::Print),
                    "pl" => Some(ReservedWord::Println),
                    "input" => Some(ReservedWord::Input),
                    "len" => Some(ReservedWord::Len),
                    "type" => Some(ReservedWord::Type),
                    "import" => Some(ReservedWord::Import),
                    "from" => Some(ReservedWord::From),
                    "as" => Some(ReservedWord::As),
                    "class" => Some(ReservedWord::Class),
                    "new" => Some(ReservedWord::New),
                    "this" => Some(ReservedWord::This),
                    "super" => Some(ReservedWord::Super),
                    _ => None,
                } {
                    self.token.push(Token::Reserved(reserved));
                } else {
                    self.token.push(Token::Identifier(identifier));
                }
            }
        }
    }
}
