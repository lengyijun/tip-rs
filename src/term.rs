use crate::ast_parser::AstNode;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Term {
    Var(Var),
    Cons(Cons),
    Mu(Mu),
}

impl Term {
    pub fn fresh_var() -> Self {
        static mut INDEX: usize = 0;
        unsafe {
            INDEX += 1;
            Term::Var(Var::FreshVarType(INDEX))
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Var {
    FreshVarType(usize),
    VarType(AstNode),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Cons {
    IntType,
    FunctionType(FunctionType),
    PointerType(PointerType),
    RecordType(RecordType),
    // used in RecordType. If a field can't infer type
    AbsentFieldType,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Mu {
    RecursiveType(RecursiveType),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct FunctionType {
    /// initial with Vec<Term::FreshVarType>
    pub params: Vec<Term>,
    /// initial with Box<Term::FreshVarType>
    pub ret: Box<Term>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct PointerType {
    pub of: Box<Term>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RecordType {
    /// initial with HashMap: x=>Term::FreshVarType
    pub fields: HashMap<String, Term>,
    /// HashMap can't be Hash
    /// so we use index to distinguish two RecordType
    index: usize,
}
impl RecordType {
    pub fn new() -> Self {
        static mut INDEX: usize = 0;
        unsafe {
            INDEX += 1;
            RecordType {
                index: INDEX,
                fields: HashMap::new(),
            }
        }
    }
}

impl Hash for RecordType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

// TODO
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RecursiveType;
