use crate::ast_parser::AstNode;
use crate::term::Cons;
use crate::term::Term;
use std::collections::HashMap;

pub struct UnionFindSolver(HashMap<Term, Term>);

impl UnionFindSolver {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn union(&mut self, k1: &Term, k2: &Term) {
        let v1: Term = self.find(k1).clone();
        let v2: Term = self.find(k2).clone();
        if v1 == v2 { return; }
        let v1_clone = v1.clone();
        let v2_clone = v2.clone();
        match (v1_clone, v2_clone) {
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
                (Cons::IntType, Cons::IntType) => {}
                (Cons::FunctionType(f1), Cons::FunctionType(f2)) => {
                    self.union(&f1.ret, &f2.ret);
                    for (p1, p2) in f1.params.iter().zip(f2.params.iter()) {
                        self.union(p1, p2);
                    }
                }
                (Cons::PointerType(p1), Cons::PointerType(p2)) => {
                    self.union(&p1.of, &p2.of);
                }
                (Cons::RecordType(r1), Cons::RecordType(r2)) => {
                    assert_eq!(r1.fields.len(), r2.fields.len());
                    for key in r1.fields.keys() {
                        self.union(&r1.fields[key], &r2.fields[key]);
                    }
                }
                (Cons::AbsentFieldType, Cons::AbsentFieldType) => {}
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

    fn find(&mut self, key: &Term) -> &Term {
        match self.0.get(key) {
            Some(par) => {
                if par == key {
                    par
                } else {
                    let par = par.clone();
                    let y = self.find(&par).clone();
                    self.0.insert(key.clone(), y);
                    self.0.get(key).unwrap()
                }
            }
            None => {
                self.0.insert(key.clone(), key.clone());
                self.0.get(key).unwrap()
            }
        }
    }

    pub fn solution(mut self) -> HashMap<Term, Term> {
        let mut h = HashMap::new();
        let keys: Vec<Term> = self.0.keys().map(|x| x.clone()).collect();
        for k in keys {
            let v = self.find(&k).clone();
            h.insert(k, v);
        }
        h
    }
}
