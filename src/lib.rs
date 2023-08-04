use pest;

use pest::error::Error;
use pest::error::ErrorVariant;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

pub enum RegisterOrNum {
    Rdi,
    Rdx,
    Rax,
    Al,
    Num(i32),
}

pub enum Operation {
    Push(RegisterOrNum),
    Pop(RegisterOrNum),
    Add(RegisterOrNum, RegisterOrNum),
    Sub(RegisterOrNum, RegisterOrNum),
    Imul(RegisterOrNum, RegisterOrNum),
    Cqo,
    Idiv(RegisterOrNum),
    Cmp(RegisterOrNum, RegisterOrNum),
    Sete(RegisterOrNum),
    Setne(RegisterOrNum),
    Setl(RegisterOrNum),
    Setle(RegisterOrNum),
    Movzb(RegisterOrNum, RegisterOrNum),
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

impl fmt::Display for RegisterOrNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rdi => write!(f, "rdi"),
            Self::Rdx => write!(f, "rdx"),
            Self::Rax => write!(f, "rax"),
            Self::Al => write!(f, "al"),
            Self::Num(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(r) => write!(f, "  push {}", r),
            Self::Pop(r) => write!(f, "  pop {}", r),
            Self::Add(r1, r2) => write!(f, "  add {}, {}", r1, r2),
            Self::Sub(r1, r2) => write!(f, "  sub {}, {}", r1, r2),
            Self::Imul(r1, r2) => write!(f, "  imul {}, {}", r1, r2),
            Self::Cqo => write!(f, "  cqo"),
            Self::Idiv(r) => write!(f, "  idiv {}", r),
            Self::Cmp(r1, r2) => write!(f, "  cmp {}, {}", r1, r2),
            Self::Sete(r) => write!(f, "  sete {}", r),
            Self::Setne(r) => write!(f, "  setne {}", r),
            Self::Setl(r) => write!(f, "  setl {}", r),
            Self::Setle(r) => write!(f, "  setle {}", r),
            Self::Movzb(r1, r2) => write!(f, "  movzb {}, {}", r1, r2),
        }
    }
}

pub fn elf_writer(path: &str, oprations: &Vec<Operation>) -> std::io::Result<()> {
    let path = Path::new(path);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    file.write_all(b".intel_syntax noprefix\n")?;
    file.write_all(b".globl main\n")?;
    file.write_all(b"main:\n")?;
    for i in oprations {
        write!(file, "{}\n", i)?;
    }
    file.write_all(b"  pop rax\n")?;
    file.write_all(b"  ret\n")
}
