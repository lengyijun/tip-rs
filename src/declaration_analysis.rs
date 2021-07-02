use crate::ast_parser::*;
use crate::dfs::Dfs;
use std::collections::HashMap;

pub struct DeclarationAnalysis {
    /// env is temporary
    env: HashMap<String, AstNode>,
    /// decl is result
    decl: HashMap<AstNode, AstNode>,
}

impl Dfs for DeclarationAnalysis {
    type ResultType = HashMap<AstNode, AstNode>;

    fn new(_: &AstNode) -> Self {
        Self {
            env: HashMap::new(),
            decl: HashMap::new(),
        }
    }

    fn visit(&mut self, node: &AstNode) -> bool {
        match node.kind {
            // only usage go to here
            // no var,paramter go to here
            AstNodeKind::Id(ref name) => {
                let root = self.env.get(name).unwrap().clone();
                self.decl.insert(node.clone(), root);
                false
            }
            AstNodeKind::Function(Function {
                ref params,
                ref vars,
                ..
            }) => {
                // because the dfs function doesn't go to parameters and vars
                // so we need to deal with them here
                for param in params {
                    if let AstNodeKind::Id(ref name) = param.kind {
                        self.env.insert(name.clone(), param.clone());
                    } else {
                        unreachable!();
                    }
                }
                for var in vars {
                    if let AstNodeKind::Id(ref name) = var.kind {
                        self.env.insert(name.clone(), var.clone());
                    } else {
                        unreachable!();
                    }
                }
                true
            }
            AstNodeKind::Program(ref functions) => {
                for function in functions {
                    if let AstNodeKind::Function(Function { ref name, .. }) = function.kind {
                        self.env.insert(name.clone(), function.clone());
                    } else {
                        unreachable!();
                    }
                }
                for function in functions {
                    let mut declaration_analysis = DeclarationAnalysis {
                        env: self.env.clone(),
                        decl: HashMap::new(),
                    };
                    declaration_analysis.dfs(function);
                    // merge
                    for (key, value) in declaration_analysis.decl {
                        self.decl.insert(key, value);
                    }
                }
                // stop dfs
                false
            }
            _ => true,
        }
    }

    fn finish(self) -> Self::ResultType {
        self.decl
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_parser::parse;
    use crate::declaration_analysis::DeclarationAnalysis;
    use crate::dfs::Dfs;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_fib_declar() -> std::io::Result<()> {
        let path = "/home/lyj/TIP/examples/fib.tip";
        let content = fs::read_to_string(&path)?;
        let program = parse(&content);
        let mut declaration_analysis = DeclarationAnalysis {
            decl: HashMap::new(),
            env: HashMap::new(),
        };
        declaration_analysis.dfs(&program);
        dbg!(declaration_analysis.decl);
        Ok(())
    }
}
