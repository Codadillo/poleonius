use std::{collections::HashMap, fmt::Display};

use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Module {
    pub ty_defs: HashMap<String, Type>,
    pub fns: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub ret_ty: Type,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmnts: Vec<Statement>,
    pub ret: Expr,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub ident: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Call(Call),
    Block(Box<Block>),
    IfElse(Box<IfElse>),
}

#[derive(Debug, Clone)]
pub struct IfElse {
    pub cond: Expr,
    pub iff: Block,
    pub elsee: Block,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: Ident,
    pub args: Vec<Expr>,
}

impl<S: Into<String>> From<S> for Ident {
    fn from(value: S) -> Self {
        Ident(value.into())
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Module {
    pub fn enum_constructors(&self) -> HashMap<String, Type> {
        self.ty_defs
            .iter()
            .filter_map(|(name, ty)| match ty {
                Type::Enum(_) => Some((name.clone(), ty.clone())),
                _ => None,
            })
            .collect()
    }
}
