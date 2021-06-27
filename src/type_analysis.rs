use crate::ast_parser::*;
use crate::term::*;
use crate::dfs::DFS;
use crate::union_find::UnionFindSolver;
use crate::field_collector::FieldCollector;
use std::collections::HashMap;

struct TypeAnalysis {
    union_find: UnionFindSolver,
    // generate from DeclarationAnalysis
    decl: HashMap<AstNode, AstNode>,
    all_fields:Vec<String>,
}

impl TypeAnalysis{
    fn astNode2Term(&self,node: &AstNode)->Term{
        match self.decl.get(node){
            Some(res)=>Term::Var(Var::VarType(res.clone())),
            None=> Term::Var(Var::VarType(node.clone()))
        }
    }
}

impl DFS for TypeAnalysis {
    fn visit(&mut self, node: &AstNode) -> bool {
        match node.kind{
    AstNodeKind::Id(_)=>{},
    AstNodeKind::DirectFieldWrite(DirectFieldWrite{})=>{
        unimplemented!();
    }
    AstNodeKind::IndirectFieldWrite(IndirectFieldWrite),
    AstNodeKind::DerefWrite(DerefWrite),
    AstNodeKind::Return(_)=>{}
    AstNodeKind::Output(Output{expr})=>{
        self.union_find.union(self.astNode2Term(expr),Term::Cons(Cons::IntType));
    }
    AstNodeKind::Error(Error)=>{}
    AstNodeKind::Assign(Assign),
    AstNodeKind::If(If{guard,..})=>{
        self.union_find.union(self.astNode2Term(guard),Term::Cons(Cons::IntType));
    }
    AstNodeKind::While(While{guard,..})=>{
        self.union_find.union(self.astNode2Term(guard),Term::Cons(Cons::IntType));
    }
    AstNodeKind::Block(_)=>{}
    AstNodeKind::Function(Function{
        ref paramters,
        ref ret,
        ..
    })=>{
        let ft=FunctionType{
            params:parameters.iter().map(self.ast2Term).collect(),
            ret: self.ast2Term(ret)
        }
        self.union_find.union(ft,self.ast2Term(node));
    }
     // AstNode::Function
    AstNodeKind::Program(_)=>{
    }
    AstNodeKind::Number(i32)=>{
        self.union_find.union(self.astNode2Term(node),Term::Cons(Cons::IntType));
    }
    AstNodeKind::Input=>{
        self.union_find.union(self.astNode2Term(node),Term::Cons(Cons::IntType));
    }
    AstNodeKind::Field(Field)=>{ }
    AstNodeKind::Record(ref fields)=>{
        let rt=RecordType::new();
        for field in fields{
            if let AstNodeKind::Field(field)=field.kind{
                rt.fields.insert(field.name,Term::Var(Var::VarType(field.exprssion)));
            }else{
                unreachable!();
            }
        }
    }
    AstNodeKind::Null=>{
        self.union_find.union(self.astNode2Term(node),Term::Cons(Cons::PointerType(PointerType{
            of: Term::freshVar()
        })));
    }
    AstNodeKind::Alloc(Alloc{expr})=>{
        self.union_find.union(self.astNode2Term(node),Term::Cons(Cons::PointerType(PointerType{
            of: self.astNode2Term(expr)
        })));
    }
    AstNodeKind::Ref(Ref)=>{
        unimplemented!();
    }
    AstNodeKind::Deref(Deref)=>{
        unimplemented!();
    }
    AstNodeKind::FunApp(FunApp{method,params})=>{
        let ft=FunctionType{
            params:vec![Term::freshVar();params.len()];
            ret: Term::freshVar()
        }
        self.union_find.union(self.astNode2Term(node),ft.ret.clone());
        self.union_find.union(self.astNode2Term(method),ft);
    }
    AstNodeKind::FieldAccess(FieldAccess{
        ref name,ref path
    })=>{
        unimplemented!();
    }
    AstNodeKind::Expression(ref BinaryOp{left,right,..})=>{
        //  left=right=node=Int
        self.union_find.union(self.astNode2Term(left),Term::Cons(Cons::IntType));
        self.union_find.union(self.astNode2Term(right),Term::Cons(Cons::IntType));
        self.union_find.union(self.astNode2Term(node),Term::Cons(Cons::IntType));
    },
    AstNodeKind::Ids(_)=>{unreachable!();}
    AstNodeKind::Vars(_)=>{unreachable!();}
        }
        true
    }
}
