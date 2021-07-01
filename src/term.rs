use crate::ast_parser::AstNode;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
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

impl From<FunctionType> for Term {
    fn from(f: FunctionType) -> Term {
        Term::Cons(Cons::FunctionType(f))
    }
}

impl From<PointerType> for Term {
    fn from(p: PointerType) -> Term {
        Term::Cons(Cons::PointerType(p))
    }
}

impl From<RecordType> for Term {
    fn from(r: RecordType) -> Term {
        Term::Cons(Cons::RecordType(r))
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

    /// from: Term::Var
    /// to: Term::Var(Var::Placeholder)
    /// used in making a RecursiveType
    pub fn substitute(&self, from: &Term, to: &Term) -> Term {
        match self {
            Term::Var(_) => {
                if self == from {
                    to.clone()
                } else {
                    self.clone()
                }
            }
            Term::Cons(c) => match c {
                Cons::IntType => self.clone(),
                Cons::FunctionType(FunctionType { params, ret }) => {
                    Term::Cons(Cons::FunctionType(FunctionType {
                        params: params.iter().map(|x| x.substitute(from, to)).collect(),
                        ret: Box::new(ret.substitute(from, to)),
                    }))
                }
                Cons::PointerType(PointerType { of }) => {
                    Term::Cons(Cons::PointerType(PointerType {
                        of: Box::new(of.substitute(from, to)),
                    }))
                }
                Cons::RecordType(RecordType { fields, .. }) => {
                    let mut r = RecordType::new();
                    for (k, v) in fields {
                        r.fields.insert(k.to_string(), v.substitute(from, to));
                    }
                    Term::Cons(Cons::RecordType(r))
                }
                Cons::AbsentFieldType => self.clone(),
            },
            Term::Mu(Mu::RecursiveType(RecursiveType { t })) => {
                if self == from {
                    to.clone()
                } else {
                    Term::Mu(Mu::RecursiveType(RecursiveType {
                        t: Box::new(t.substitute(from, to)),
                    }))
                }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Var {
    FreshVarType(usize),
    VarType(AstNode),
    // used only in RecursiveType
    PlaceHolder,
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
            Var::PlaceHolder => {
                f.write_fmt(format_args!("Placeholder"))?;
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
    /// t: Term::Var(..)
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
        if self.params.is_empty() {
            f.write_fmt(format_args!("( )->{:?}", self.ret))?;
        } else {
            f.write_char('(')?;
            for x in self.params.iter().take(self.params.len() - 1) {
                f.write_fmt(format_args!("{:?},", x))?;
            }
            f.write_fmt(format_args!("{:?}", self.params.last().unwrap()))?;
            f.write_fmt(format_args!(")->{:?}", self.ret))?;
        }
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

#[derive(Clone)]
pub struct RecordType {
    /// initial with HashMap: x=>Term::FreshVarType
    pub fields: HashMap<String, Term>,
    /// HashMap can't be Hash
    /// so we use index to distinguish two RecordType
    index: usize,
}
impl PartialEq for RecordType {
    fn eq(&self, other: &RecordType) -> bool {
        self.fields == other.fields
    }
}

impl Eq for RecordType {}

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

impl fmt::Debug for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.fields))?;
        Ok(())
    }
}

/// RecursiveType will use a Placeholder
/// `x => ⭡(Placeholder)` means x = ⭡x = ⭡⭡x = ....
/// `x => (Placeholder,IntType)-> IntType` means
/// x = ((..,IntType)->IntType,IntType) -> IntType
///
/// This design will help to write unit test
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RecursiveType {
    pub t: Box<Term>,
}
