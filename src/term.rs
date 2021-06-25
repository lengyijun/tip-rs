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
    /// initial with Vec<Term::FreshVarType>
    pub params: Vec<Term>,
    /// initial with Box<Term::FreshVarType>
    pub ret: Box<Term>,
}

struct PointerType {
    pub of: Box<Term>,
}

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

// TODO
struct AbsentType;

// TODO
struct RecursiveType;
