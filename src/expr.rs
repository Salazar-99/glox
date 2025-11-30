use crate::error::GloxError;
use crate::interpreter::Visitor;
use crate::token::{Literal, Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Unary(Unary),
    Literal(LiteralExpr),
}

impl Expr {
    pub fn accept(&self, visitor: impl Visitor) -> Result<Literal, GloxError> {
        match self {
            Expr::Grouping(x) => visitor.visit_grouping(x),
            Expr::Unary(x) => visitor.visit_unary(x),
            Expr::Binary(x) => visitor.visit_binary(x),
            Expr::Literal(x) => visitor.visit_literal(x),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

// LiteralExpr to avoid name overloading
// with TokenType::Literal -> Literal from parser
#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}
