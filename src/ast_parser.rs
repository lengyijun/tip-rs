use pest::Parser;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;

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

pub fn parse(input: &str) {
    let pairs = IdentParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        let a = build_ast_from_expr(pair);
        dbg!(a);
    }
}

#[derive(Debug)]
struct DirectFieldWrite {
    id: String,
    field: String,
}

#[derive(Debug)]
struct IndirectFieldWrite {
    // Box<AstNode<Expression>>
    expr: Box<AstNode>,
    field: String,
}

#[derive(Debug)]
struct DerefWrite {
    // Box<AstNode::Atom>
    expr: Box<AstNode>,
}

#[derive(Debug)]
struct Return {
    // Box<AstNode<Expression>>
    expr: Box<AstNode>,
}

#[derive(Debug)]
struct Output {
    // Box<AstNode<Expression>>
    expr: Box<AstNode>,
}

#[derive(Debug)]
struct Error {
    // Box<AstNode<Expression>>
    expr: Box<AstNode>,
}

#[derive(Debug)]
struct Assign {
    /// AstNode::Id, AstNode::DirectFieldWrite, AstNode::IndirectFieldWrite, AstNode::DerefWrite
    left: Box<AstNode>,
    /// AstNode::Expression
    right: Box<AstNode>,
}

#[derive(Debug)]
struct If {
    /// AstNode::Expression
    guard: Box<AstNode>,
    /// most likely AstNode::block
    if_block: Box<AstNode>,
    /// most likely AstNode::block
    else_block: Option<Box<AstNode>>,
}

#[derive(Debug)]
struct While {
    /// AstNode::Expression
    guard: Box<AstNode>,
    /// most likely AstNode::block
    block: Box<AstNode>,
}

#[derive(Debug)]
struct Block {
    exprs: Vec<AstNode>,
}

#[derive(Debug)]
struct Function {
    name: String,
    parameters: Box<AstNode>,
    /// AstNode::Vars
    vars: Box<AstNode>,
    statements: Vec<AstNode>,
    /// AstNode::Return
    ret: Box<AstNode>,
}

#[derive(Debug)]
struct Field {
    id: String,
    /// AstNode::Expression
    expression: Box<AstNode>,
}

#[derive(Debug)]
struct Alloc {
    /// AstNode::Expression
    expr: Box<AstNode>,
}

#[derive(Debug)]
struct Ref {
    /// AstNode::Id
    id: Box<AstNode>,
}

#[derive(Debug)]
struct Deref {
    /// AstNode::Expression
    atom: Box<AstNode>,
}

#[derive(Debug)]
struct FunApp {
    method: Box<AstNode>,
    /// AstNode::Expression
    params: Vec<AstNode>,
}

#[derive(Debug)]
struct FieldAccess {
    name: Box<AstNode>,
    path: Vec<String>,
}

#[derive(Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Gt,
    Equal,
}

#[derive(Debug)]
struct BinaryOp {
    op: Op,
    /// AstNode::Atom or AstNode::Expression
    left: Box<AstNode>,
    right: Box<AstNode>,
}

#[derive(Debug)]
pub struct AstNode {
    kind: AstNodeKind,
    /// position
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub enum AstNodeKind {
    Id(String),
    /// AstNode::Id
    Ids(Vec<AstNode>),
    DirectFieldWrite(DirectFieldWrite),
    IndirectFieldWrite(IndirectFieldWrite),
    DerefWrite(DerefWrite),
    /// AstNode::Id
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
                    id: pair.next().unwrap().as_str().into(),
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
            let parameters = Box::new(build_ast_from_expr(pair.next().unwrap()));
            let vars = Box::new(build_ast_from_expr(pair.next().unwrap()));
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
