use crate::token::{self, Literal, Token, TokenType};
use crate::expr::{Binary, Expr, Grouping, LiteralExpr, Unary};

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
        let equality_types: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token_type(&equality_types) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Binary { left: Box::new(expr), operator: operator, right: Box::new(right)});
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        let comparison_types: [TokenType; 4] = [TokenType::Greater, TokenType:: GreaterEqual, TokenType::Less, TokenType::LessEqual];
        while self.match_token_type(&comparison_types) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary(Binary { left: Box::new(expr), operator:operator, right: Box::new(right) });
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        let term_types: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
        while self.match_token_type(&term_types) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary(Binary { left: Box::new(expr), operator:operator, right: Box::new(right) });
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        let factor_types: [TokenType; 2] = [TokenType::Slash, TokenType::Star];
        while self.match_token_type(&factor_types) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary(Binary { left: Box::new(expr), operator:operator, right: Box::new(right) });
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        let unary_types: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
        if self.match_token_type(&unary_types) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary(Unary { operator:operator, right: Box::new(right) });
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token_type(&[TokenType::False]) {
            return Expr::Literal(LiteralExpr{value: Literal::Bool})
        } else if self.match_token_type(&[TokenType::True]) {
            return Expr::Literal(LiteralExpr{value: Literal::Bool})
        } else if self.match_token_type(&[TokenType::Nil]) {
            return Expr::Literal(LiteralExpr{value: Literal::Nil})
        } else if self.match_token_type(&[TokenType::String, TokenType::Number]) {
            return Expr::Literal(LiteralExpr { value: self.previous().literal})
        } else if self.match_token_type(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Grouping{expression: Box::new(expr)});
        }
        
        panic!("Unexpected token")
    }
    
    fn peek(&mut self) -> TokenType {
        return self.tokens.get(self.current).unwrap().token_type;
    }

    fn previous(&mut self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek() == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous()
    }

    fn check(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false
        }
        return self.peek() == typ
    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            if self.check(*typ) {
                self.advance();
                return true
            }
        }
        return false
    }
}