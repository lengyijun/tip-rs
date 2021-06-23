use crate::ast_parser::AstNode;
use std::collections::HashMap;

pub enum Term {
    Var(Var),
    Cons(Cons),
    Mu(Mu),
}

enum Var {
    FreshVarType(FreshVarType),
    VarType(VarType),
}

pub enum Cons {
    IntType(IntType),
    FunctionType(FunctionType),
    PointerType(PointerType),
    RecordType(RecordType),
    AbsentType(AbsentType),
}

enum Mu {
    RecursiveType(RecursiveType),
}

struct VarType(AstNode);
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

struct IntType;

struct FunctionType {
    pub params: Vec<Term>,
    pub ret: Box<Term>,
}

struct PointerType {
    pub of: Box<Term>,
}

struct RecordType {
    pub fields: HashMap<String, Term>,
}

// TODO
struct AbsentType;

// TODO
struct RecursiveType;
