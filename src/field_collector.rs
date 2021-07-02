use crate::ast_parser::*;
use crate::dfs::Dfs;
use std::collections::HashSet;

pub struct FieldCollector {
    fields: HashSet<String>,
}

impl Dfs for FieldCollector {
    type ResultType = Vec<String>;

    fn new(_: &AstNode) -> Self {
        Self {
            fields: HashSet::new(),
        }
    }

    fn visit(&mut self, node: &AstNode) -> bool {
        if let AstNodeKind::Record(fs) = &node.kind {
            for f in fs {
                self.fields.insert(f.name.clone());
            }
        }

        true
    }

    fn finish(self) -> Self::ResultType {
        self.fields.into_iter().collect::<Vec<String>>()
    }
}
