use crate::binary::{Operation, RegisterOrNum};
use pest;
use pest::error::Error;
use pest::error::ErrorVariant;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct CalcParser;

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

fn get_operator(rule: Rule) -> Op {
    match rule {
        Rule::addop => Op::Add,
        Rule::subop => Op::Sub,
        Rule::mulop => Op::Mul,
        Rule::divop => Op::Div,
        Rule::eqop => Op::Eq,
        Rule::nqop => Op::Neq,
        Rule::ltop => Op::Lt,
        Rule::leop => Op::Le,
        Rule::gtop => Op::Gt,
        Rule::geop => Op::Ge,
        _ => {
            panic!()
        }
    }
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Result<Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::equation => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap())?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap())?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::relational => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap())?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap())?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap())?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap())?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::factor => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap())?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap())?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::unary => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            match content.as_rule() {
                Rule::atom => build_ast(content),
                _ => Ok(Expr::BinOp {
                    lhs: Box::new(Expr::Integer(0)),
                    op: get_operator(content.as_rule()),
                    rhs: Box::new(build_ast(inner.next().unwrap())?),
                }),
            }
        }
        Rule::atom => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            match content.as_rule() {
                Rule::num => Ok(Expr::Integer(content.as_str().parse::<i32>().unwrap())),
                Rule::equation => build_ast(content),
                _ => {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: String::from("innerError when parsing atom"),
                        },
                        content.as_span(),
                    ))
                }
            }
        }
        _ => {
            return Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: String::from("innerError"),
                },
                pair.as_span(),
            ));
        }
    }
}

pub fn source_to_ast(source: &str) -> Result<Expr, Error<Rule>> {
    let pair = CalcParser::parse(Rule::main, source)?.next().unwrap();
    build_ast(pair.into_inner().next().unwrap())
}

impl Expr {
    fn to_assembly_inner(&self, out: &mut Vec<Operation>) -> () {
        use Operation::*;
        use RegisterOrNum::*;
        match self {
            Expr::Integer(n) => {
                out.push(Push(Num(n.clone())));
            }
            Expr::BinOp { lhs, op, rhs } => {
                lhs.to_assembly_inner(out);
                rhs.to_assembly_inner(out);
                out.push(Pop(Rdi));
                out.push(Pop(Rax));
                match op {
                    Op::Add => out.push(Add(Rax, Rdi)),
                    Op::Sub => out.push(Sub(Rax, Rdi)),
                    Op::Mul => out.push(Imul(Rax, Rdi)),
                    Op::Div => {
                        out.push(Cqo);
                        out.push(Idiv(Rdi))
                    }
                    Op::Eq => {
                        out.push(Cmp(Rax, Rdi));
                        out.push(Sete(Al));
                        out.push(Movzb(Rax, Al))
                    }
                    Op::Neq => {
                        out.push(Cmp(Rax, Rdi));
                        out.push(Setne(Al));
                        out.push(Movzb(Rax, Al))
                    }
                    Op::Lt => {
                        out.push(Cmp(Rax, Rdi));
                        out.push(Setl(Al));
                        out.push(Movzb(Rax, Al))
                    }
                    Op::Le => {
                        out.push(Cmp(Rax, Rdi));
                        out.push(Setle(Al));
                        out.push(Movzb(Rax, Al))
                    }
                    Op::Gt => {
                        out.push(Cmp(Rdi, Rax));
                        out.push(Setl(Al));
                        out.push(Movzb(Rax, Al))
                    }
                    Op::Ge => {
                        out.push(Cmp(Rdi, Rax));
                        out.push(Setle(Al));
                        out.push(Movzb(Rax, Al))
                    }
                }
                out.push(Push(Rax));
            }
        }
    }
    pub fn to_assembly(&self) -> Vec<Operation> {
        let mut out = vec![];
        self.to_assembly_inner(&mut out);
        out
    }
}
