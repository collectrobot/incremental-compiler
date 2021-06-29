use crate::types::{IdString};
use super::token::{Token};

#[derive(Clone, Debug, PartialEq)]
pub struct LetBinding {
    pub identifier: IdString,
    pub expr: AstNode
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Int(i64),
    Prim {op: IdString, args: Vec<AstNode>},

    Let {
        bindings: Vec<LetBinding>,
        body: Box<AstNode>
    },

    Var { name: IdString },
    Error { msg: IdString, token: Token },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub info: (),
    pub exp: AstNode,
}