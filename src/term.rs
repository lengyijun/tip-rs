use crate::ast_parser::AstNode;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug,Hash,Eq,PartialEq)]
pub enum Term {
    Var(Var),
    Cons(Cons),
    Mu(Mu),
}

#[derive(Debug,Hash,Eq,PartialEq)]
enum Var {
    FreshVarType(FreshVarType),
    VarType(VarType),
}

#[derive(Debug,Hash,Eq,PartialEq)]
pub enum Cons {
    IntType(IntType),
    FunctionType(FunctionType),
    PointerType(PointerType),
    RecordType(RecordType),
    AbsentFieldType(AbsentFieldType),
}

#[derive(Debug,Hash,Eq,PartialEq)]
enum Mu {
    RecursiveType(RecursiveType),
}

#[derive(Debug,Hash,Eq,PartialEq)]
struct VarType(AstNode);

#[derive(Debug,Hash,Eq,PartialEq)]
struct FreshVarType(usize);
impl FreshVarType {
    fn new() -> Self {
        static mut index: usize = 0;
        unsafe {
            index += 1;
            Self(index)
        }
    }
}

#[derive(Debug,Hash,Eq,PartialEq)]
struct IntType;

#[derive(Debug,Hash,Eq,PartialEq)]
struct FunctionType {
    /// initial with Vec<Term::FreshVarType>
    pub params: Vec<Term>,
    /// initial with Box<Term::FreshVarType>
    pub ret: Box<Term>,
}

#[derive(Debug,Hash,Eq,PartialEq)]
struct PointerType {
    pub of: Box<Term>,
}

#[derive(Debug,Eq,PartialEq)]
struct RecordType {
    /// initial with HashMap: x=>Term::FreshVarType
    pub fields: HashMap<String, Term>,
    /// HashMap can't be Hash
    /// so we use index to distinguish two RecordType
    index: usize,
}
impl RecordType {
    fn new(input: Vec<String>) -> Self {
        static mut index: usize = 0;
        let mut fields = HashMap::new();
        for x in input {
            fields.insert(x, Term::Var(Var::FreshVarType(FreshVarType::new())));
        }
        unsafe {
            index += 1;
            RecordType { index, fields }
        }
    }
}

impl Hash for RecordType {
    fn hash<H: Hasher>(&self, state: &mut H) {
            self.index.hash(state);
        }
}

// used in RecordType. If a field can't infer type
#[derive(Debug,Hash,Eq,PartialEq)]
struct AbsentFieldType;

// TODO
#[derive(Debug,Hash,Eq,PartialEq)]
struct RecursiveType;
