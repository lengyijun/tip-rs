use crate::ast_parser::AstNode;
use crate::term::Cons;
use crate::term::Term;
use std::collections::BTreeMap;

pub struct UnionFind(BTreeMap<Term, Term>);

impl UnionFind {
    pub fn union(&mut self, k1: Term, k2: Term) {
        let v1 = match self.find(k1) {
            Some(v1) => v1,
            None => {
                self.0.insert(k1, k1);
                k1
            }
        };
        let v2 = match self.find(k1) {
            Some(v2) => v2,
            None => {
                self.0.insert(k2, k2);
                k2
            }
        };
        match (v1, v2) {
            (Term::Var(_), Term::Var(_)) => {
                self.0.insert(v1, v2);
            }
            (Term::Var(_), _) => {
                self.0.insert(v1, v2);
            }
            (_, Term::Var(_)) => {
                self.0.insert(v2, v1);
            }
            (Term::Cons(c1), Term::Cons(c2)) => match (c1, c2) {
                (Cons::IntType(i1), Cons::IntType(i2)) => {}
                (Cons::FunctionType(f1), Cons::FunctionType(f2)) => {
                    self.union(f1.ret, f2.ret);
                    for (p1, p2) in f1.params.iter().zip(f1.params.iter()) {
                        self.union(p1, p2);
                    }
                }
                (Cons::PointerType(p1), Cons::PointerType(p2)) => {
                    self.union(p1.of, p2.of);
                }
                (Cons::RecordType(r1), Cons::RecordType(r2)) => {
                    assert!(r1.fields.len() == r2.fields.len());
                    for key in r1.fields.keys() {
                        self.union(r1.fields[key], r2.fields[key]);
                    }
                }
                (Cons::AbsentType(a1), Cons::AbsentType(a2)) => {}
                (_, _) => {
                    unreachable!();
                }
            },
            // cons->mu
            // mu->mu
            (_, _) => {
                unreachable!();
            }
        };
    }

    // TODO
    fn find(&self) {}
}
