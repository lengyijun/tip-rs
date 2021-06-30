use crate::ast_parser::*;
use crate::declaration_analysis::DeclarationAnalysis;
use crate::dfs::DFS;
use crate::field_collector::FieldCollector;
use crate::term::*;
use crate::union_find::UnionFindSolver;
use std::collections::{HashMap, HashSet};

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
                name,
                ..
            }) => {
                let ft = if name == "main" {
                    FunctionType {
                        params: parameters.iter().map(|x| self.astNode2Term(x)).collect(),
                        ret: Box::new(Term::Cons(Cons::IntType)),
                    }
                } else {
                    FunctionType {
                        params: parameters.iter().map(|x| self.astNode2Term(x)).collect(),
                        ret: Box::new(self.astNode2Term(ret)),
                    }
                };
                self.union_find.union(
                    &self.astNode2Term(node),
                    &Term::Cons(Cons::FunctionType(ft)),
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
                let params_output: Vec<Term> =
                    params.iter().map(|x| self.astNode2Term(x)).collect();
                let ft = FunctionType {
                    params: params_output,
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
        let env = self.union_find.solution();
        let mut res: HashMap<Term, Term> = HashMap::new();
        let mut fresh_vars = HashMap::<Term, Term>::new();
        for (k, v) in &env {
            if let Term::Var(Var::VarType(n)) = k {
                if let AstNodeKind::Id(_) = n.kind {
                    let x = close(&v, &env, &mut fresh_vars);
                    res.insert(k.clone(), x);
                } else if let AstNodeKind::Function(_) = n.kind {
                    let x = close(&v, &env, &mut fresh_vars);
                    res.insert(k.clone(), x);
                }
            }
        }
        res
    }
}

fn close_rec(
    t: &Term,
    env: &HashMap<Term, Term>,
    fresh_vars: &mut HashMap<Term, Term>,
    mut visited: HashSet<Term>,
) -> Term {
    match t {
        Term::Var(var) => {
            let (t_par, b) = match env.get(t) {
                Some(t_par) => (Some(t_par), t_par != t),
                None => (None, false),
            };
            match (visited.get(t), b) {
                (None, true) => {
                    visited.insert(t.clone());
                    let cterm = close_rec(t_par.unwrap(), env, fresh_vars, visited);
                    if let Some(f) = fresh_vars.get(t) {
                        if let Term::Cons(ref c) = cterm {
                            if c.contain(f) {
                                unimplemented!();
                                // TODO
                                // recursive type
                            }
                        }
                    }
                    cterm
                }
                _ => match fresh_vars.get(t) {
                    Some(res) => res.clone(),
                    None => {
                        fresh_vars.insert(t.clone(), Term::fresh_var());
                        fresh_vars.get(t).unwrap().clone()
                    }
                },
            }
        }
        Term::Cons(c) => match c {
            Cons::IntType => Term::Cons(Cons::IntType),
            Cons::AbsentFieldType => Term::Cons(Cons::AbsentFieldType),
            Cons::FunctionType(ft) => {
                let mut params = vec![];
                for p in &ft.params {
                    params.push(close_rec(p, env, fresh_vars, visited.clone()));
                }
                Term::Cons(Cons::FunctionType(FunctionType {
                    params,
                    ret: Box::new(close_rec(&ft.ret, env, fresh_vars, visited.clone())),
                }))
            }
            Cons::PointerType(PointerType { ref of }) => {
                let pt_clone = PointerType {
                    of: Box::new(close_rec(of, env, fresh_vars, visited.clone())),
                };
                Term::Cons(Cons::PointerType(pt_clone))
            }
            Cons::RecordType(RecordType { fields, .. }) => {
                let mut res = RecordType::new();
                for (k, v) in fields {
                    res.fields.insert(
                        k.to_string(),
                        close_rec(v, env, fresh_vars, visited.clone()),
                    );
                }
                Term::Cons(Cons::RecordType(res))
            }
        },
        Term::Mu(_) => {
            unimplemented!();
        }
    }
}

fn close(t: &Term, env: &HashMap<Term, Term>, fresh_vars: &mut HashMap<Term, Term>) -> Term {
    close_rec(t, env, fresh_vars, HashSet::new())
}

#[cfg(test)]
mod tests {
    use crate::ast_parser::parse;
    use crate::dfs::DFS;
    use crate::type_analysis::TypeAnalysis;
    use std::fs;

    #[test]
    fn test_single_type_analysis() -> std::io::Result<()> {
        // let path = "/home/lyj/TIP/examples/fib.tip";
        // let path = "/home/lyj/TIP/examples/mono2.tip";
        let path = "/home/lyj/TIP/examples/foo.tip";
        let content = fs::read_to_string(&path)?;
        let program = parse(&content);
        let res = TypeAnalysis::work(&program);
        dbg!(res);
        Ok(())
    }

    #[test]
    fn test_type_analysis() -> std::io::Result<()> {
        for entry in fs::read_dir("/home/lyj/TIP/examples")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let content = &fs::read_to_string(&path)?;
                dbg!(&path);
                let program = parse(&content);
                let res = TypeAnalysis::work(&program);
            }
        }
        Ok(())
    }
}
