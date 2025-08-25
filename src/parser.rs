use crate::token::{Token, TokenType};
use crate::expr::{Expr, Binary};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let token_types: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token_type(&token_types) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            });
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {

    }
    fn previous(&mut self) -> Token {

    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        return true
    }
}