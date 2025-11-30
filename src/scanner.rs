use std::collections::HashMap;

use crate::token::{self, Literal, Token, TokenType};
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let tokens: Vec<Token> = Vec::<Token>::new();
        let chars: Vec<char> = source.chars().collect();
        let keywords: HashMap<String, TokenType> = token::get_keywords();
        Scanner {
            source: source,
            chars: chars,
            tokens: tokens,
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let final_token = Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: Literal::Nil,
            line: self.line,
        };
        self.tokens.push(final_token);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        if self.is_at_end() {
            return;
        }
        let c: char = self.advance();
        match c {
            // Single character symbols
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            // One or two character symbols
            '!' => {
                if self.check('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.check('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.check('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.check('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            // Check for comments
            '/' => {
                if self.check('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            // Ignore whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            // New lines
            '\n' => self.line += 1,
            // String literals
            '"' => self.handle_string(),
            // Numbers, identifiers, and keywords
            _ => {
                if self.is_digit(c) {
                    self.handle_number();
                } else if self.is_alpha(c) {
                    self.handle_identifier();
                } else {
                    eprintln!("Unexpected character: {c}")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        // Get the current character then move the index up by one
        let current_char = self.chars[self.current];
        self.current += 1;
        return current_char;
    }

    fn check(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.chars[self.current] != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current]
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current + 1]
    }

    fn is_at_end(&mut self) -> bool {
        if self.current == self.chars.len() {
            return true;
        }
        return false;
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alphanumeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Literal::Nil);
    }

    // For handling floats and bools
    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: self.line,
        })
    }

    fn add_token_string(&mut self) {
        // Trim the quotes
        let lexeme = self.source[self.start + 1..self.current - 1].to_string();
        self.tokens.push(Token {
            token_type: TokenType::String,
            lexeme: lexeme.clone(),
            literal: Literal::Str(lexeme),
            line: self.line,
        })
    }

    fn handle_string(&mut self) {
        // Handle multi-line strings
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        // Error for no closing quote
        if self.is_at_end() {
            eprintln!("Untermintated string.")
        }
        // Handle closing quote
        self.advance();
        // Add the token
        self.add_token_string();
    }

    fn handle_number(&mut self) {
        // Handle initial digits
        let mut peek: char = self.peek();
        while self.is_digit(peek) {
            self.advance();
            peek = self.peek();
        }

        // Check for a decimal and keep going if numbers follow it
        let peek_next: char = self.peek_next();
        if self.peek() == '.' && self.is_digit(peek_next) {
            self.advance();
            let mut peek: char = self.peek();
            while self.is_digit(peek) {
                self.advance();
                peek = self.peek();
            }
        }

        let number = self.source[self.start..self.current].to_string();
        let float_value = number.parse::<f32>().unwrap();
        self.add_token_literal(TokenType::Number, Literal::Float(float_value));
    }

    fn handle_identifier(&mut self) {
        let mut peek: char = self.peek();
        while self.is_alphanumeric(peek) {
            self.advance();
            peek = self.peek();
        }
        let text: String = self.source[self.start..self.current].to_string();
        // Try to match the text to a keyword otherwise it's an Identifier
        let token_type: TokenType = self
            .keywords
            .get(&text)
            .copied()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }
}
