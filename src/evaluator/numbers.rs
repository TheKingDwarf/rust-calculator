#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Exponent,
    Multiply,
    Divide,
    Subtract,
    Add,
    LeftParenthesis,
    RightParenthesis,
}

impl Operand {
    #[allow(dead_code)]
    pub fn priority(&self) -> i32 {
        match *self {
            Operand::Exponent => 3,
            Operand::Multiply => 2,
            Operand::Divide => 2,
            Operand::Subtract => 1,
            Operand::Add => 1,
            Operand::LeftParenthesis => -1,
            Operand::RightParenthesis => -1,
        }
    }

    pub fn is_parenthesis(&self) -> bool {
        match *self {
            Operand::LeftParenthesis | Operand::RightParenthesis => true,
            _ => false,
        }
    }
}

#[allow(unused_imports)]
pub use Operand::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub symbol: char,
    pub power: f64,
    pub coefficient: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub values: Vec<Types>,
    pub operation: Operand,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Float(f64),
    Fraction(Fraction),
    Variable(Variable),
    Expression(Expression),
}
pub use Types::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionComponents {
    Type(Types),
    Op(Operand),
}

pub use ExpressionComponents::*;
