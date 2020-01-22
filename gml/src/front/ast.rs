use crate::symbol::Symbol;
use crate::front::Span;

pub use crate::front::action_ast::*;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    Error(Expr),

    Assign((Option<Op>, Span), Box<(Expr, Span)>, Box<(Expr, Span)>),
    Invoke(Call),
    Declare(Declare, Box<[(Symbol, Span)]>),
    Block(Box<[(Stmt, Span)]>),

    If(Box<(Expr, Span)>, Box<(Stmt, Span)>, Option<Box<(Stmt, Span)>>),
    Repeat(Box<(Expr, Span)>, Box<(Stmt, Span)>),
    While(Box<(Expr, Span)>, Box<(Stmt, Span)>),
    Do(Box<(Stmt, Span)>, Box<(Expr, Span)>),
    For(Box<(Stmt, Span)>, Box<(Expr, Span)>, Box<(Stmt, Span)>, Box<(Stmt, Span)>),
    With(Box<(Expr, Span)>, Box<(Stmt, Span)>),
    Switch(Box<(Expr, Span)>, Box<[(Stmt, Span)]>),

    Jump(Jump),
    Return(Box<(Expr, Span)>),
    Case(Option<Box<(Expr, Span)>>),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Declare {
    Local,
    Global,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Jump {
    Break,
    Continue,
    Exit,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Error,
    Value(Value),
    Unary((Unary, Span), Box<(Expr, Span)>),
    Binary((Binary, Span), Box<(Expr, Span)>, Box<(Expr, Span)>),
    Field(Box<(Expr, Span)>, (Symbol, Span)),
    Index(Box<(Expr, Span)>, Box<[(Expr, Span)]>),
    Call(Call),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Value {
    Ident(Symbol),
    Real(f64),
    String(Symbol),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Unary {
    Positive,
    Negate,
    Invert,
    BitInvert,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Binary {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,

    And,
    Or,
    Xor,

    Op(Op),

    Div,
    Mod,

    ShiftLeft,
    ShiftRight,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,

    BitAnd,
    BitOr,
    BitXor,
}

#[derive(PartialEq, Debug)]
pub struct Call(pub (Symbol, Span), pub Box<[(Expr, Span)]>);
