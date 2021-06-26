use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(gt, Left) | Operator::new(equal, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
        ])
    };
}

#[derive(Parser)]
#[grammar = "tip.pest"]
struct IdentParser;

pub fn parse(input: &str) -> AstNode{
    let pair = IdentParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    let a = build_ast_from_expr(pair);
    a
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct DirectFieldWrite {
    // AstNode::Id
    pub id: Box<AstNode>,
    pub field: String,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct IndirectFieldWrite {
    // Box<AstNode<Expression>>
    pub expr: Box<AstNode>,
    pub field: String,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct DerefWrite {
    // Box<AstNode::Atom>
    pub expr: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Return {
    // Box<AstNode<Expression>>
    pub expr: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Output {
    // Box<AstNode<Expression>>
    pub expr: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Error {
    // Box<AstNode<Expression>>
    pub expr: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Assign {
    /// AstNode::Id, AstNode::DirectFieldWrite, AstNode::IndirectFieldWrite, AstNode::DerefWrite
    pub left: Box<AstNode>,
    /// AstNode::Expression
    pub right: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct If {
    /// AstNode::Expression
    pub guard: Box<AstNode>,
    /// most likely AstNode::block
    pub if_block: Box<AstNode>,
    /// most likely AstNode::block
    pub else_block: Option<Box<AstNode>>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct While {
    /// AstNode::Expression
    pub guard: Box<AstNode>,
    /// most likely AstNode::block
    pub block: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Block {
    pub exprs: Vec<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    /// Vec<AstNode::Id>
    pub parameters: Vec<AstNode>,
    /// Vec<AstNode::Id>
    pub vars: Vec<AstNode>,
    pub statements: Vec<AstNode>,
    /// AstNode::Return
    pub ret: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Field {
    pub id: String,
    /// AstNode::Expression
    pub expression: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Alloc {
    /// AstNode::Expression
    pub expr: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Ref {
    /// AstNode::Id
    pub id: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Deref {
    /// AstNode::Expression
    pub atom: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct FunApp {
    pub method: Box<AstNode>,
    /// AstNode::Expression
    pub params: Vec<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct FieldAccess {
    pub name: Box<AstNode>,
    pub path: Vec<String>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Gt,
    Equal,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct BinaryOp {
    pub op: Op,
    /// AstNode::Atom or AstNode::Expression
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AstNode {
    pub kind: AstNodeKind,
    /// start position
    /// note: different AstNode may share same start position
    line: usize,
    col: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum AstNodeKind {
    Id(String),
    /// only temporary, for convenient
    /// will not be used in DFS
    /// Vec<AstNode::Id>
    Ids(Vec<AstNode>),
    DirectFieldWrite(DirectFieldWrite),
    IndirectFieldWrite(IndirectFieldWrite),
    DerefWrite(DerefWrite),
    /// only temporary
    /// Vec<AstNode::Id>
    Vars(Vec<AstNode>),
    Return(Return),
    Output(Output),
    Error(Error),
    Assign(Assign),
    If(If),
    While(While),
    Block(Block),
    Function(Function),
    /// AstNode::Function
    Program(Vec<AstNode>),
    Number(i32),
    Input,
    Field(Field),
    /// AstNode::Field
    Record(Vec<AstNode>),
    Null,
    Alloc(Alloc),
    Ref(Ref),
    Deref(Deref),
    FunApp(FunApp),
    FieldAccess(FieldAccess),
    Expression(BinaryOp),
}

// use Precedence Climbing Method to parse AstNode::Expression
fn parse_expression(expression: Pairs<Rule>) -> AstNode {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::expression => parse_expression(pair.into_inner()),
            _ => build_ast_from_expr(pair),
        },
        |lhs: AstNode, op: Pair<Rule>, rhs: AstNode| {
            let (line, col) = (lhs.line, lhs.col);
            match op.as_rule() {
                Rule::add => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Add,
                    }),
                },
                Rule::subtract => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Subtract,
                    }),
                },
                Rule::multiply => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Multiply,
                    }),
                },
                Rule::divide => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Divide,
                    }),
                },

                Rule::equal => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Equal,
                    }),
                },
                Rule::gt => AstNode {
                    line,
                    col,
                    kind: AstNodeKind::Expression(BinaryOp {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Op::Gt,
                    }),
                },
                _ => unreachable!(),
            }
        },
    )
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    // dbg!(pair.as_str());

    let (line, col) = pair.as_span().start_pos().line_col();
    match pair.as_rule() {
        Rule::id => AstNode {
            kind: AstNodeKind::Id(pair.as_str().into()),
            line,
            col,
        },
        Rule::ids => AstNode {
            line,
            col,
            kind: AstNodeKind::Ids(pair.into_inner().map(build_ast_from_expr).collect()),
        },
        Rule::directFieldWrite => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::DirectFieldWrite(DirectFieldWrite {
                    id: Box::new(build_ast_from_expr(pair.next().unwrap())),
                    field: pair.next().unwrap().as_str().into(),
                }),
            }
        }
        Rule::indirectFieldWrite => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::IndirectFieldWrite(IndirectFieldWrite {
                    expr: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                    field: pair.next().unwrap().as_str().into(),
                }),
            }
        }
        Rule::derefWrite => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::DerefWrite(DerefWrite {
                    expr: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                }),
            }
        }
        Rule::vars => AstNode {
            line,
            col,
            kind: AstNodeKind::Vars(
                pair.into_inner()
                    .map(build_ast_from_expr)
                    .map(|x| {
                        if let AstNodeKind::Ids(y) = x.kind {
                            y.into_iter()
                        } else {
                            unreachable!();
                        }
                    })
                    .flatten()
                    .collect(),
            ),
        },
        Rule::return_expr => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::Return(Return {
                    expr: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                }),
            }
        }
        Rule::output => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::Output(Output {
                    expr: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                }),
            }
        }
        Rule::error => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::Error(Error {
                    expr: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                }),
            }
        }
        Rule::assign => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::Assign(Assign {
                    left: Box::new(build_ast_from_expr(pair.next().unwrap())),
                    right: Box::new(build_ast_from_expr(pair.next().unwrap())),
                }),
            }
        }
        Rule::if_expr => {
            let mut pair = pair.into_inner();
            let guard = Box::new(parse_expression(pair.next().unwrap().into_inner()));
            let if_block = Box::new(build_ast_from_expr(pair.next().unwrap()));
            let else_block = pair
                .next()
                .map_or_else(|| None, |x| Some(Box::new(build_ast_from_expr(x))));
            AstNode {
                line,
                col,
                kind: AstNodeKind::If(If {
                    guard,
                    if_block,
                    else_block,
                }),
            }
        }
        Rule::while_expr => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::While(While {
                    guard: Box::new(parse_expression(pair.next().unwrap().into_inner())),
                    block: Box::new(build_ast_from_expr(pair.next().unwrap())),
                }),
            }
        }
        Rule::block => AstNode {
            line,
            col,
            kind: AstNodeKind::Block(Block {
                exprs: pair.into_inner().map(build_ast_from_expr).collect(),
            }),
        },
        Rule::function => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str().to_string();
            let ids = Box::new(build_ast_from_expr(pair.next().unwrap()));
            let parameters = if let AstNodeKind::Ids(parameters) = ids.kind {
                parameters
            } else {
                unreachable!();
            };
            let vars = Box::new(build_ast_from_expr(pair.next().unwrap()));
            let vars = if let AstNodeKind::Vars(vars) = vars.kind {
                vars
            } else {
                unreachable!();
            };
            let mut statements: Vec<AstNode> = pair.map(build_ast_from_expr).collect();
            let ret = Box::new(statements.pop().unwrap());
            AstNode {
                line,
                col,
                kind: AstNodeKind::Function(Function {
                    name,
                    parameters,
                    vars,
                    statements,
                    ret,
                }),
            }
        }
        Rule::program => AstNode {
            line,
            col,
            kind: AstNodeKind::Program(
                pair.into_inner()
                    .filter(|x| x.as_rule() != Rule::EOI)
                    .map(build_ast_from_expr)
                    .collect(),
            ),
        },
        Rule::number => AstNode {
            line,
            col,
            kind: AstNodeKind::Number(pair.as_str().parse().unwrap()),
        },
        Rule::input => AstNode {
            line,
            col,
            kind: AstNodeKind::Input,
        },
        Rule::field => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::Field(Field {
                    id: pair.next().unwrap().as_str().to_string(),
                    expression: Box::new(build_ast_from_expr(pair.next().unwrap())),
                }),
            }
        }
        Rule::record => AstNode {
            line,
            col,
            kind: AstNodeKind::Record(pair.into_inner().map(build_ast_from_expr).collect()),
        },
        Rule::null => AstNode {
            line,
            col,
            kind: AstNodeKind::Null,
        },
        Rule::alloc => AstNode {
            line,
            col,
            kind: AstNodeKind::Alloc(Alloc {
                expr: Box::new(parse_expression(pair.into_inner())),
            }),
        },
        Rule::ref_expr => AstNode {
            line,
            col,
            kind: AstNodeKind::Ref(Ref {
                id: Box::new(build_ast_from_expr(pair.into_inner().next().unwrap())),
            }),
        },
        Rule::deref => AstNode {
            line,
            col,
            kind: AstNodeKind::Deref(Deref {
                atom: Box::new(build_ast_from_expr(pair.into_inner().next().unwrap())),
            }),
        },
        Rule::funApp => {
            let mut pair = pair.into_inner();
            let method = Box::new(build_ast_from_expr(pair.next().unwrap()));
            let params: Vec<AstNode> = pair.map(build_ast_from_expr).collect();
            AstNode {
                line,
                col,
                kind: AstNodeKind::FunApp(FunApp { method, params }),
            }
        }
        Rule::fieldAccess => {
            let mut pair = pair.into_inner();
            AstNode {
                line,
                col,
                kind: AstNodeKind::FieldAccess(FieldAccess {
                    name: Box::new(build_ast_from_expr(pair.next().unwrap())),
                    path: pair.map(|x| x.as_str().to_string()).collect(),
                }),
            }
        }
        Rule::expression => parse_expression(pair.into_inner()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::Rule;
    use crate::ast_parser::build_ast_from_expr;
    use crate::ast_parser::parse;
    use crate::ast_parser::IdentParser;
    use crate::pest::Parser;

    use std::fs;

    #[test]
    fn test_parse() -> std::io::Result<()> {
        for entry in fs::read_dir("/home/lyj/TIP/examples")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let content = &fs::read_to_string(&path)?;
                // dbg!(&path);
                parse(&content);
            }
        }
        Ok(())
    }

    #[test]
    fn test_fib() -> std::io::Result<()> {
        let path = "/home/lyj/TIP/examples/fib.tip";
        let content = &fs::read_to_string(&path)?;
        parse(&content);
        Ok(())
    }

    #[test]
    fn test_mountain_climbing() -> std::io::Result<()> {
        let content = "1+2*3";
        let pairs =
            IdentParser::parse(Rule::expression, content).unwrap_or_else(|e| panic!("{}", e));
        for pair in pairs {
            let a = build_ast_from_expr(pair);
            dbg!(a);
        }
        Ok(())
    }
}
