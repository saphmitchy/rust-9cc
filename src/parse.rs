use crate::binary::{Operation, RegisterOrNum};
use pest;
use pest::error::Error;
use pest::error::ErrorVariant;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct CalcParser;

#[derive(Debug)]
pub enum Expr {
    Var {
        name: String,
        offset: usize,
    },
    Integer(i32),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Return {
        expr: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
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
    Assign,
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
        Rule::asnop => Op::Assign,
        _ => {
            panic!()
        }
    }
}

fn build_ast(
    pair: pest::iterators::Pair<Rule>,
    env: &mut HashMap<String, usize>,
) -> Result<Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::assign => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::equation => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::relational => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
                    }
                } else {
                    break;
                }
            }
            Ok(ret)
        }
        Rule::factor => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
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
                Rule::atom => build_ast(content, env),
                _ => Ok(Expr::BinOp {
                    lhs: Box::new(Expr::Integer(0)),
                    op: get_operator(content.as_rule()),
                    rhs: Box::new(build_ast(inner.next().unwrap(), env)?),
                }),
            }
        }
        Rule::atom => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            match content.as_rule() {
                Rule::ident => {
                    let name = String::from(content.as_str());
                    match env.get(&name) {
                        Some(offset) => Ok(Expr::Var {
                            name,
                            offset: offset.clone(),
                        }),
                        None => {
                            let offset = (env.len() + 1) * 8;
                            env.insert(name.clone(), offset);
                            let new_var = Expr::Var { name, offset };
                            Ok(new_var)
                        }
                    }
                }
                Rule::num => Ok(Expr::Integer(content.as_str().parse::<i32>().unwrap())),
                Rule::assign => build_ast(content, env),
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
        Rule::res => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            let expr = Box::new(build_ast(content, env)?);
            Ok(Expr::Return { expr })
        }
        _ => {
            // println!("{:?}", pair.as_rule());
            return Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: String::from("innerError"),
                },
                pair.as_span(),
            ));
        }
    }
}

pub fn source_to_ast(source: &str) -> Result<Vec<Expr>, Error<Rule>> {
    let pair = CalcParser::parse(Rule::main, source)?.next().unwrap();
    let mut env = HashMap::new();
    let mut v = pair
        .into_inner()
        .into_iter()
        .map(|x| build_ast(x, &mut env))
        .collect::<Vec<_>>();
    v.pop();
    v.into_iter().collect::<Result<_, _>>()
}

impl Expr {
    fn gen_lval(&self, out: &mut Vec<Operation>) -> () {
        use Operation::*;
        use RegisterOrNum::*;
        match self {
            Expr::Var { name: _, offset } => {
                out.push(Mov(Rax, Rbp));
                out.push(Sub(Rax, Num(offset.clone() as i32)));
                out.push(Push(Rax));
            }
            _ => panic!("代入の左辺値が変数ではありません"),
        }
    }

    fn to_assembly_inner(&self, out: &mut Vec<Operation>) -> () {
        use Operation::*;
        use RegisterOrNum::*;
        match self {
            Expr::Var { name: _, offset: _ } => {
                self.gen_lval(out);
                out.push(Pop(Rax));
                out.push(Load(Rax, Rax));
                out.push(Push(Rax));
            }
            Expr::Integer(n) => {
                out.push(Push(Num(n.clone())));
            }
            Expr::BinOp { lhs, op, rhs } => {
                if *op == Op::Assign {
                    lhs.gen_lval(out);
                    rhs.to_assembly_inner(out);
                    out.push(Pop(Rdi));
                    out.push(Pop(Rax));
                    out.push(Store(Rax, Rdi));
                    out.push(Push(Rdi));
                    return;
                }
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
                    Op::Assign => panic!(),
                }
                out.push(Push(Rax));
            }
            Expr::Return { expr } => {
                expr.to_assembly_inner(out);
                out.push(Pop(Rax));
                out.push(Mov(Rsp, Rbp));
                out.push(Pop(Rbp));
                out.push(Ret);
            }
        }
    }
    pub fn to_assembly(&self) -> Vec<Operation> {
        let mut out = vec![];
        self.to_assembly_inner(&mut out);
        out.push(Operation::Pop(RegisterOrNum::Rax));
        out
    }
}
