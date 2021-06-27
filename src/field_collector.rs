use crate::ast_parser::*;
use crate::dfs::DFS;
use std::collections::HashSet;

pub struct FieldCollector {
    fields: HashSet<String>,
}

impl DFS for FieldCollector {
    type ResultType = Vec<String>;

    fn new(_: &AstNode) -> Self {
        Self {
            fields: HashSet::new(),
        }
    }

    fn visit(&mut self, node: &AstNode) -> bool {
        match &node.kind {
            AstNodeKind::Record(fs) => {
                for f in fs {
                    self.fields.insert(f.name.clone());
                }
            }
            _ => {}
        }
        true
    }

    fn finish(self) -> Self::ResultType {
        self.fields.into_iter().collect::<Vec<String>>()
    }
}
