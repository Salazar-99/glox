use crate::error::GloxError;
use crate::expr::{Binary, Grouping, Unary};
use crate::expr::{Expr, LiteralExpr};
use crate::token::{Literal, TokenType};

pub trait Visitor {
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<Literal, GloxError>;
    fn visit_grouping(&self, expr: &Grouping) -> Result<Literal, GloxError>;
    fn visit_unary(&self, expr: &Unary) -> Result<Literal, GloxError>;
    fn visit_binary(&self, expr: &Binary) -> Result<Literal, GloxError>;
}

#[derive(Copy, Clone)]
pub struct Interpreter {}

impl Visitor for Interpreter {
    fn visit_literal(&self, expr: &LiteralExpr) -> Result<Literal, GloxError> {
        match &expr.value {
            Literal::Bool(b) => Ok(Literal::Bool(*b)),
            Literal::Float(f) => Ok(Literal::Float(*f)),
            Literal::Str(s) => Ok(Literal::Str(s.clone())),
            Literal::Nil => Ok(Literal::Nil),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Result<Literal, GloxError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary(&self, expr: &Unary) -> Result<Literal, GloxError> {
        let right: Literal = self.evaluate(&expr.right)?;
        match &expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Float(f) => return Ok(Literal::Float(-1.0 * f)),
                _ => Err(GloxError::RuntimeError(
                    "Tried to minus something other than a number big bro".to_string(),
                )),
            },
            TokenType::Bang => return Ok(Literal::Bool(!self.is_truthy(right))),
            _ => Err(GloxError::RuntimeError(
                "Called visit_unary on something that isn't a unary expression".to_string(),
            )),
        }
    }

    fn visit_binary(&self, expr: &Binary) -> Result<Literal, GloxError> {
        let left: Literal = self.evaluate(&expr.left)?;
        let right: Literal = self.evaluate(&expr.right)?;

        match &expr.operator.token_type {
            TokenType::Minus => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l - r)),
                _ => Err(GloxError::RuntimeError(
                    "Tried to subtract something other than two floats".to_string(),
                )),
            },
            TokenType::Plus => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l + r)),
                (Literal::Str(l), Literal::Str(r)) => Ok(Literal::Str(l + &r)),
                _ => Err(GloxError::RuntimeError(
                    "Tried to add two things that weren't either both floats or strings"
                        .to_string(),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l / r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do / between two floats big bro".to_string(),
                )),
            },
            TokenType::Star => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l * r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do * between two floats big bro".to_string(),
                )),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l > r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do > between two floats big bro".to_string(),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l >= r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do >= for two floats big bro".to_string(),
                )),
            },

            TokenType::Less => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l < r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do < for two floats big bro".to_string(),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l < r)),
                _ => Err(GloxError::RuntimeError(
                    "You can only do < for two floats big bro".to_string(),
                )),
            },
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(left, right))),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(left, right))),
            _ => Err(GloxError::RuntimeError(
                "Tried to call visit_binary on something that isn't a binary expression"
                    .to_string(),
            )),
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: &Expr) -> Result<Literal, GloxError> {
        let value: Literal = self.evaluate(expr)?;
        Ok(value)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, GloxError> {
        let value: Literal = expr.accept(self.clone())?;
        Ok(value)
    }

    fn is_truthy(&self, value: Literal) -> bool {
        match value {
            Literal::Nil => {
                return false;
            }
            Literal::Bool(b) => return b,
            _ => return true,
        }
    }

    fn is_equal(&self, l: Literal, r: Literal) -> bool {
        match (l, r) {
            (Literal::Float(a), Literal::Float(b)) => a == b,
            (Literal::Str(a), Literal::Str(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}
