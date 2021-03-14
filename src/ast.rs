use crate::token::{Token};

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Int(i64),
    Prim {op: String, args: Vec<AstNode>},
    Let { bindings: Vec<(String, AstNode)>, body: Box<AstNode> },
    Var { name: String },
    Error { msg: String, token: Token },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub info: (),
    pub exp: AstNode,
}