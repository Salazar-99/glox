use crate::token::{Literal, Token};

// TODO: Generate this Enum and its members with generate_ast
#[derive(Clone, Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Unary(Unary),
    Literal(LiteralExpr)
}

#[derive(Clone, Debug)]
pub struct Grouping { 
    pub expression: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>
}

// LiteralExpr to avoid name overloading
// with TokenType::Literal -> Literal from parser
#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub value: Literal
}