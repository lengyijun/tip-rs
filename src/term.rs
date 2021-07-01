use crate::ast_parser::AstNode;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Term {
    Var(Var),
    Cons(Cons),
    Mu(Mu),
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => {
                f.write_fmt(format_args!("{:?}", v))?;
            }
            Term::Cons(c) => {
                f.write_fmt(format_args!("{:?}", c))?;
            }
            Term::Mu(m) => {
                f.write_fmt(format_args!("{:?}", m))?;
            }
        }
        Ok(())
    }
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

// TODO 为了Mu,把FreshVarType提炼出来？
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Var {
    FreshVarType(usize),
    VarType(AstNode),
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Var::FreshVarType(id) => {
                f.write_fmt(format_args!("x{}", id))?;
            }
            Var::VarType(node) => {
                f.write_fmt(format_args!("{:?}", node))?;
            }
        }
        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Cons {
    IntType,
    FunctionType(FunctionType),
    PointerType(PointerType),
    RecordType(RecordType),
    // used in RecordType. If a field can't infer type
    AbsentFieldType,
}

impl Cons {
    /// t is Term::Var(..)
    pub fn contain(&self, t: &Term) -> bool {
        match self {
            Cons::IntType => false,
            Cons::FunctionType(FunctionType {
                ref params,
                ref ret,
            }) => {
                if t == (ret as &Term) {
                    return true;
                }
                if let Term::Cons(c) = t {
                    if c.contain(t) {
                        return true;
                    }
                }
                for p in params {
                    if p == t {
                        return true;
                    }
                    if let Term::Cons(c) = p {
                        if c.contain(t) {
                            return true;
                        }
                    }
                }
                false
            }
            Cons::PointerType(PointerType { ref of }) => {
                if t == (of as &Term) {
                    return true;
                }
                if let Term::Cons(c) = of as &Term {
                    if c.contain(t) {
                        return true;
                    }
                }
                false
            }
            Cons::RecordType(RecordType { ref fields, .. }) => {
                for f in fields.values() {
                    if f == t {
                        return true;
                    }
                    if let Term::Cons(c) = f {
                        if c.contain(t) {
                            return true;
                        }
                    }
                }
                false
            }
            Cons::AbsentFieldType => false,
        }
    }
}

impl fmt::Debug for Cons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons::IntType => {
                f.write_str("IntType")?;
            }
            Cons::FunctionType(ft) => {
                f.write_fmt(format_args!("{:?}", ft))?;
            }
            Cons::PointerType(pt) => {
                f.write_fmt(format_args!("{:?}", pt))?;
            }
            Cons::RecordType(rt) => {
                f.write_fmt(format_args!("{:?}", rt))?;
            }
            Cons::AbsentFieldType => {
                f.write_str("AbsentFieldType")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Mu {
    RecursiveType(RecursiveType),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct FunctionType {
    /// initial with Vec<Term::FreshVarType>
    pub params: Vec<Term>,
    /// initial with Box<Term::FreshVarType>
    pub ret: Box<Term>,
}

impl fmt::Debug for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({:?})->{:?}", self.params, self.ret))?;
        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct PointerType {
    pub of: Box<Term>,
}

impl fmt::Debug for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("⭡{:?}", self.of))?;
        Ok(())
    }
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RecursiveType {
    // freshvar
    pub v: Var,
    pub t: Box<Term>,
}
