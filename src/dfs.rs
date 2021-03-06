use crate::ast_parser::*;

pub trait Dfs {
    type ResultType;

    // visit children
    fn dfs(&mut self, node: &AstNode) {
        if !self.visit(node) {
            return;
        }
        match node.kind {
            AstNodeKind::Id(_) => {}
            AstNodeKind::DirectFieldWrite(_) => {}
            AstNodeKind::IndirectFieldWrite(IndirectFieldWrite { ref expr, .. }) => {
                self.dfs(expr);
            }
            AstNodeKind::DerefWrite(DerefWrite { ref expr }) => {
                self.dfs(expr);
            }
            AstNodeKind::Output(Output { ref expr }) => {
                self.dfs(expr);
            }
            AstNodeKind::Error(Error { ref expr }) => {
                self.dfs(expr);
            }
            AstNodeKind::Assign(Assign {
                ref left,
                ref right,
            }) => {
                self.dfs(left);
                self.dfs(right);
            }
            AstNodeKind::If(If {
                ref guard,
                ref if_block,
                ref else_block,
            }) => {
                self.dfs(guard);
                self.dfs(if_block);
                if let Some(ref else_block) = else_block {
                    self.dfs(else_block);
                }
            }
            AstNodeKind::While(While {
                ref guard,
                ref block,
            }) => {
                self.dfs(guard);
                self.dfs(block);
            }
            AstNodeKind::Block(Block { ref exprs }) => {
                for expr in exprs {
                    self.dfs(expr);
                }
            }
            AstNodeKind::Function(Function {
                ref params,
                ref vars,
                ref statements,
                ref ret,
                ..
            }) => {
                // just some Id, no need to dfs
                // for parameter in parameters {
                // self.dfs(parameter);
                // }
                // for var in vars {
                // self.dfs(var);
                // }
                for statement in statements {
                    self.dfs(statement);
                }
                self.dfs(ret);
            }
            // AstNode::Function
            AstNodeKind::Program(ref functions) => {
                for function in functions {
                    self.dfs(function);
                }
            }
            AstNodeKind::Number(_) => {}
            AstNodeKind::Input => {}
            AstNodeKind::Record(ref fields) => {
                for field in fields {
                    self.dfs(&field.expression);
                }
            }
            AstNodeKind::Null => {}
            AstNodeKind::Alloc(Alloc { ref expr }) => {
                self.dfs(expr);
            }
            AstNodeKind::Ref(Ref { ref id }) => {
                self.dfs(id);
            }
            AstNodeKind::Deref(Deref { ref atom }) => {
                self.dfs(atom);
            }
            AstNodeKind::FunApp(FunApp {
                ref method,
                ref params,
            }) => {
                self.dfs(method);
                for param in params {
                    self.dfs(param);
                }
            }
            AstNodeKind::FieldAccess(FieldAccess { ref name, .. }) => {
                self.dfs(name);
            }
            AstNodeKind::Expression(BinaryOp {
                ref left,
                ref right,
                ..
            }) => {
                self.dfs(left);
                self.dfs(right);
            }
        }
    }

    /// return true: continue dfs
    /// return false: stop dfs
    fn visit(&mut self, node: &AstNode) -> bool;

    fn finish(self) -> Self::ResultType;

    fn work(node: &AstNode) -> Self::ResultType
    where
        Self: Sized,
    {
        let mut m = Self::new(node);
        m.dfs(node);
        m.finish()
    }

    fn new(node: &AstNode) -> Self;
}
