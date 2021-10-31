use std::collections::HashMap;

use crate::frontend::ast::{AstNode};

pub type IdString = std::rc::Rc<String>;

pub struct Environment {
    map: HashMap<IdString, AstNode>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            map: HashMap::new(),
        }
    }

    pub fn exists(&self, id: IdString) -> bool {
        self.map.contains_key(&*id)
    }

    pub fn insert(&mut self, id: IdString, val: AstNode) {
        self.map.insert(id, val);
    }

    pub fn get(&self, id: IdString) -> Option<&AstNode> {
        self.map.get(&*id)
    }

    pub fn get_value_of(&self, id: IdString) -> Option<&AstNode> {
        let v = self.get(id);
        match v {
            Some(&AstNode::Var{ ref name }) => {
                self.get_value_of(name.clone())
            },

            Some(&AstNode::Int(_)) => {
                v
            },

            _ => {
                None
            }
        }
    }
}