use std::collections::HashMap;

pub fn get_keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("class".to_string(), TokenType::Class);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("fun".to_string(), TokenType::Fun);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("nil".to_string(), TokenType::Nil);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("print".to_string(), TokenType::Print);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords.insert("super".to_string(), TokenType::Super);
    keywords.insert("this".to_string(), TokenType::This);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("var".to_string(), TokenType::Var);
    keywords.insert("while".to_string(), TokenType::While);
    keywords
}

#[derive(Debug, Clone)]
pub enum Literal {
    Float,
    Str,
    Bool,
    Nil
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line
        }
    }

    pub fn to_string(&self) {
        let token_type = &self.token_type;
        let lexeme = &self.lexeme;
        let literal = &self.literal;
        println!("Type: {:#?}, Lexeme: {:#?}, Literal: {:#?}", token_type, lexeme, literal)
    }
}

// TokenType contains all of the elements in the lexical grammar
// of the Lox language.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special
    EOF
}
