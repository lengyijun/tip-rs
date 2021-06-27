use crate::ast_parser::*;
use crate::declaration_analysis::DeclarationAnalysis;
use crate::dfs::DFS;
use crate::field_collector::FieldCollector;
use crate::term::*;
use crate::union_find::UnionFindSolver;
use std::collections::HashMap;

struct TypeAnalysis {
    union_find: UnionFindSolver,
    // generate from DeclarationAnalysis
    decl: HashMap<AstNode, AstNode>,
    all_fields: Vec<String>,
}

impl TypeAnalysis {
    fn astNode2Term(&self, node: &AstNode) -> Term {
        match self.decl.get(node) {
            Some(res) => Term::Var(Var::VarType(res.clone())),
            None => Term::Var(Var::VarType(node.clone())),
        }
    }

    fn new_record(&self) -> RecordType {
        let mut rec = RecordType::new();
        for field in &self.all_fields {
            rec.fields.insert(field.clone(), Term::fresh_var());
        }
        rec
    }
}

impl DFS for TypeAnalysis {
    type ResultType = HashMap<Term, Term>;

    fn new(node: &AstNode) -> Self {
        let all_fields = FieldCollector::work(node);
        let decl = DeclarationAnalysis::work(node);
        Self {
            union_find: UnionFindSolver::new(),
            all_fields,
            decl,
        }
    }

    fn visit(&mut self, node: &AstNode) -> bool {
        match &node.kind {
            AstNodeKind::Id(_) => {}
            AstNodeKind::DirectFieldWrite(_) => {}
            AstNodeKind::IndirectFieldWrite(_) => {}
            AstNodeKind::DerefWrite(_) => {}
            AstNodeKind::Return(_) => {}
            AstNodeKind::Output(Output { expr }) => {
                self.union_find
                    .union(&self.astNode2Term(&expr), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::Error(_) => {}
            AstNodeKind::Assign(Assign {
                ref left,
                ref right,
                ..
            }) => {
                match &left.kind {
                    AstNodeKind::Id(_) => {
                        self.union_find
                            .union(&self.astNode2Term(left), &self.astNode2Term(right));
                    }
                    AstNodeKind::DirectFieldWrite(DirectFieldWrite { field, id }) => {
                        let mut rec = self.new_record();
                        rec.fields.insert(field.clone(), self.astNode2Term(right));
                        self.union_find
                            .union(&self.astNode2Term(&id), &Term::Cons(Cons::RecordType(rec)));
                    }
                    AstNodeKind::IndirectFieldWrite(IndirectFieldWrite {
                        ref expr,
                        ref field,
                    }) => {
                        let mut rec = self.new_record();
                        rec.fields.insert(field.clone(), self.astNode2Term(right));
                        self.union_find.union(
                            &self.astNode2Term(expr),
                            &Term::Cons(Cons::PointerType(PointerType {
                                of: Box::new(Term::Cons(Cons::RecordType(rec))),
                            })),
                        );
                    }
                    // *c=f
                    AstNodeKind::DerefWrite(DerefWrite { ref expr }) => {
                        self.union_find.union(
                            &self.astNode2Term(expr),
                            &Term::Cons(Cons::PointerType(PointerType {
                                of: Box::new(self.astNode2Term(right)),
                            })),
                        );
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            AstNodeKind::If(If { ref guard, .. }) => {
                self.union_find
                    .union(&self.astNode2Term(guard), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::While(While { ref guard, .. }) => {
                self.union_find
                    .union(&self.astNode2Term(guard), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::Block(_) => {}
            AstNodeKind::Function(Function {
                ref parameters,
                ref ret,
                ..
            }) => {
                let ft = FunctionType {
                    params: parameters.iter().map(|x| self.astNode2Term(x)).collect(),
                    ret: Box::new(self.astNode2Term(ret)),
                };
                self.union_find.union(
                    &Term::Cons(Cons::FunctionType(ft)),
                    &self.astNode2Term(node),
                );
            }
            AstNodeKind::Program(_) => {}
            AstNodeKind::Number(_) => {
                self.union_find
                    .union(&self.astNode2Term(node), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::Input => {
                self.union_find
                    .union(&self.astNode2Term(node), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::Record(ref fields) => {
                let mut rec = self.new_record();
                for field in fields {
                    rec.fields
                        .insert(field.name.clone(), self.astNode2Term(&field.expression));
                }
                self.union_find
                    .union(&self.astNode2Term(node), &Term::Cons(Cons::RecordType(rec)));
            }
            AstNodeKind::Null => {
                self.union_find.union(
                    &self.astNode2Term(node),
                    &Term::Cons(Cons::PointerType(PointerType {
                        of: Box::new(Term::fresh_var()),
                    })),
                );
            }
            AstNodeKind::Alloc(Alloc { ref expr }) => {
                self.union_find.union(
                    &self.astNode2Term(node),
                    &Term::Cons(Cons::PointerType(PointerType {
                        of: Box::new(self.astNode2Term(expr)),
                    })),
                );
            }
            AstNodeKind::Ref(Ref { ref id }) => {
                self.union_find.union(
                    &self.astNode2Term(node),
                    &Term::Cons(Cons::PointerType(PointerType {
                        of: Box::new(self.astNode2Term(id)),
                    })),
                );
            }
            AstNodeKind::Deref(Deref { ref atom }) => {
                self.union_find.union(
                    &self.astNode2Term(atom),
                    &Term::Cons(Cons::PointerType(PointerType {
                        of: Box::new(self.astNode2Term(node)),
                    })),
                );
            }
            AstNodeKind::FunApp(FunApp {
                ref method,
                ref params,
            }) => {
                let ft = FunctionType {
                    params: vec![Term::fresh_var(); params.len()],
                    ret: Box::new(Term::fresh_var()),
                };
                self.union_find.union(&self.astNode2Term(node), &ft.ret);
                self.union_find.union(
                    &self.astNode2Term(method),
                    &Term::Cons(Cons::FunctionType(ft)),
                );
            }
            AstNodeKind::FieldAccess(FieldAccess { ref name, ref path }) => {
                let mut rec = self.new_record();
                rec.fields.insert(path.to_string(), self.astNode2Term(name));
                self.union_find
                    .union(&self.astNode2Term(node), &Term::Cons(Cons::RecordType(rec)));
            }
            AstNodeKind::Expression(BinaryOp {
                ref left,
                ref right,
                ..
            }) => {
                //  left=right=node=Int
                self.union_find
                    .union(&self.astNode2Term(left), &Term::Cons(Cons::IntType));
                self.union_find
                    .union(&self.astNode2Term(right), &Term::Cons(Cons::IntType));
                self.union_find
                    .union(&self.astNode2Term(node), &Term::Cons(Cons::IntType));
            }
            AstNodeKind::Ids(_) => {
                unreachable!();
            }
            AstNodeKind::Vars(_) => {
                unreachable!();
            }
        }
        true
    }

    fn finish(self) -> Self::ResultType {
        self.union_find.solution()
    }
}
