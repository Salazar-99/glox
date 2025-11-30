use crate::error::GloxError;

use crate::expr::{Binary, Expr, Grouping, LiteralExpr, Unary};
use crate::token::{Literal, Token, TokenType};

// A Recursive Decent Parser
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, GloxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, GloxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, GloxError> {
        let mut expr = self.comparison()?;
        let equality_types: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token_type(&equality_types) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, GloxError> {
        let mut expr: Expr = self.term()?;
        let comparison_types: [TokenType; 4] = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_token_type(&comparison_types) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, GloxError> {
        let mut expr: Expr = self.factor()?;
        let term_types: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
        while self.match_token_type(&term_types) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, GloxError> {
        let mut expr: Expr = self.unary()?;
        let factor_types: [TokenType; 2] = [TokenType::Slash, TokenType::Star];
        while self.match_token_type(&factor_types) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, GloxError> {
        let unary_types: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
        if self.match_token_type(&unary_types) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator: operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, GloxError> {
        if self.match_token_type(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Bool(false),
            }));
        } else if self.match_token_type(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Bool(true),
            }));
        } else if self.match_token_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            }));
        } else if self.match_token_type(&[TokenType::String, TokenType::Number]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        } else if self.match_token_type(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }));
        }

        Err(GloxError::UnexpectedToken(
            self.peek_lexeme(),
            self.current_line().try_into().unwrap(),
        ))
    }

    fn peek(&mut self) -> TokenType {
        return self.tokens.get(self.current).unwrap().token_type;
    }

    fn peek_lexeme(&mut self) -> String {
        return self.tokens.get(self.current).unwrap().lexeme.clone();
    }

    fn current_line(&self) -> usize {
        if self.current < self.tokens.len() {
            self.tokens[self.current].line as usize
        } else if !self.tokens.is_empty() {
            self.tokens[self.tokens.len() - 1].line as usize
        } else {
            1
        }
    }

    fn previous(&mut self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek() == TokenType::EOF;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous();
    }

    fn check(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek() == typ;
    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            if self.check(*typ) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<Token, GloxError> {
        if self.check(typ) {
            Ok(self.advance())
        } else {
            Err(GloxError::UnexpectedToken(
                message.to_string(),
                self.current_line().try_into().unwrap(),
            ))
        }
    }

    // fn synchronize(&mut self) {
    //     self.advance();
    //     while !self.is_at_end() {
    //         if self.previous().token_type == TokenType::Semicolon {
    //             return;
    //         }
    //     }
    //     match self.peek() {
    //         TokenType::Class
    //         | TokenType::For
    //         | TokenType::If
    //         | TokenType::Print
    //         | TokenType::Return
    //         | TokenType::Var
    //         | TokenType::While
    //         | TokenType::Fun => return,
    //         _ => {
    //             self.advance();
    //         }
    //     }
    // }
}
