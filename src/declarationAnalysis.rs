use crate::ast_parser::*;
use crate::dfs::DFS;
use std::collections::HashMap;

struct DeclarationAnalysis {
    /// env is temporary
    env: HashMap<String, AstNode>,
    /// decl is result
    decl: HashMap<AstNode, AstNode>,
}

impl DFS for DeclarationAnalysis {
    fn visit(&mut self, node: &AstNode)->bool {
        match node.kind {
            // only usage go to here
            // no declaration go to here
            AstNodeKind::Id(ref name) => {
                let root= self.env.get(name).unwrap().clone();
                self.decl.insert(node.clone(), root);
                return false;
            }
            AstNodeKind::Function(Function {
                ref parameters,
                ref vars,
                ..
            }) => {
                for parameter in parameters {
                    if let AstNodeKind::Id(ref name) = parameter.kind {
                        self.env.insert(name.clone(), parameter.clone());
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
                return true;
            }
            /// AstNode::Function
            AstNodeKind::Program(ref functions) => {
                for function in functions {
                    if let AstNodeKind::Function(Function{ref name,..})=function.kind{
                        self.env.insert(name.clone(), node.clone());
                    }else{
                        unreachable!();
                    }
                }
                for function in functions {
                    let mut declarationAnalysis = DeclarationAnalysis {
                        env: self.env.clone(),
                        decl: HashMap::new(),
                    };
                    declarationAnalysis.dfs(function);
                    // merge
                    for (key, value) in declarationAnalysis.decl {
                        self.decl.insert(key, value);
                    }
                }
                // stop dfs
                return false;
            }
            _ => {
                return true;
            }
        }
    }
}
