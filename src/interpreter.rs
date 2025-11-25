use crate::expr::{LiteralExpr};
use crate::token::{Literal};

trait Visitor {
    fn visit_literal(&self, expr: &LiteralExpr) -> Literal;
    fn visit_grouping(&self, expr: &Grouping) -> Literal;
    fn visit_unary(&self, expr: &Unary) -> Literal;
    fn visit_binary(&self, expr: &Binary) -> Litaral;
}

pub struct Interpreter {

}

impl Visitor for Interpreter {
    fn visit_literal(&self, expr: &LiteralExpr) -> Literal {
        match &expr.value {
            Literal::Bool(b) => {
                return *b
            },
            Literal::Float(f) => Literal::Float(*f),
            Literal::Str(s) => Literal::Str(s.clone()),
            Literal::Nil => Literal::Nil,
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Literal {
        return self.evaluate(expr);
    }

    fn visit_unary(&self, expr: &Unary) -> Literal {
        right: Literal = self.evaluate(expr.right);
        match &expr.operator.token_type {
            TokenType::Minus => {
                match right => {
                    Literal::Float(f) {
                        -1 * f
                    }
                    _ => {
                        Err(GloxError::RuntimeError("Tried to minus something other than a number big bro"))
                    }
                }
            }
            TokenType::Bang => {
                return !self.is_truthy(right);
            }        
        }
    }

    fn visit_binary(&self, expr: &Binary) -> Literal {
        left: Literal = self.evaluate(expr.left);
        right: Literal = self.evaluate(expr.right);

        match &expr.operator.token_type {
            TokenType::Minus => {
                match (left, right) {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l - r                                            
                    }
                    _ => {
                        return ParsingError
                    }
                }
            }
            TokenType::Plus => {
                match(left, right) {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l + r
                    }
                    (Literal::String(l), Literal::String(r)) => {
                        l.push_str(r)
                    }
                    _ => {
                        return ParsingError
                    }
                }
            }
            TokenType::Slash => {
                match (left, right) {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l / r           
                    }
                    _ => {
                        return ParsingError
                    } 
                }
            }
            TokenType::Star -> {
                match (left, right) => {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l * r
                    }
                    - => {
                        return ParsingError
                    }
                }
            }
            TokenType::Greater => {
                match (left, right) => {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l > r
                    }
                    _ => {
                        GloxError::ParsingError{}
                    }
                }
            }
            TokenType::GreaterEqual => {
                match (left, right) => {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l >= r
                    }
                    _ => {
                        GloxError::RuntimeError(String.new("You can only do >= for two floats big bro"))
                    }
                }
            }
            
            TokenType::Less => {
                match (left, right) => {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l < r
                    }
                    _ => {
                        GloxError::RuntimeError(String.new("You can only do < for two floats big bro"))
                    }
                }
            } 
            TokenType::LessEqual => {
                match (left, right) => {
                    (Literal::Float(l), Literal::Float(r)) => {
                        l < r
                    }
                    _ => {
                        GloxError::RuntimeError(String.new("You can only do < for two floats big bro"))
                    }
                }
            }
            TokenType::BangEqual => {
                !self.is_equal(left, right)
            }
            TokenType::EqualEqual => {
                self.is_equal(left, right)
            }
        }
        
        return Literal::Nil;
    }
}

impl Visitor {
    fn interpret(&self, expr: Expr) -> Result<Literal, GloxError::RuntimeError> {
        value: Literal = self.evaluate(expr)?;
        Ok(value)
    }

    fn evaluate(&self, expr: Expr) -> Result<Literal, GloxError::RuntimeError> {
        value: Literal = expr.accept(&self)?;
        Ok(value)
    }

    fn is_truthy(&self, value Literal) {
        match value {
            Literal.Nil => {
                return false
            }
            Literal.Bool(b) => {
                return *b
            }
            _ => {
                return true
            }
        }   
    }

    fn is_equal(&self, l: Literal, r: Literal) {
        if matches!(left, Literal::Nil) && matches!(right, Literal::Nil) {
            true
        } else if matches!(left, Literal::Nil) {
            false
        } else {
            match(l, r) {
                (Literal::Float(a), Literal::Float(b)) => a == b,
                (Literal::Str(a), Literal::Str(b)) => a == b,
                (Literal::Bool(a), Literal::Bool(b)) => a == b,
                _ => GloxError::RuntimeError(String.new("You tried to compare two values of different types"))
            }
        }
    }
}
