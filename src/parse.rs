use crate::ast::*;

use crate::typing::{get_type, Type};

use pest;
use pest::error::Error;
use pest::error::ErrorVariant;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct CalcParser;

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

fn build_ast_from_expr(
    pair: pest::iterators::Pair<Rule>,
    env: &mut HashMap<String, ValInfo>,
) -> Result<Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::assign | Rule::equation | Rule::relational | Rule::addminus | Rule::factor => {
            let mut inner = pair.into_inner();
            let mut ret = build_ast_from_expr(inner.next().unwrap(), env)?;
            loop {
                if let Some(op) = inner.next() {
                    ret = Expr::BinOp {
                        lhs: Box::new(ret),
                        op: get_operator(op.as_rule()),
                        rhs: Box::new(build_ast_from_expr(inner.next().unwrap(), env)?),
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
                Rule::atom => build_ast_from_expr(content, env),
                Rule::addr => Ok(Expr::Addr(Box::new(build_ast_from_expr(
                    inner.next().unwrap(),
                    env,
                )?))),
                Rule::deref => Ok(Expr::Dref(Box::new(build_ast_from_expr(
                    inner.next().unwrap(),
                    env,
                )?))),
                _ => Ok(Expr::BinOp {
                    lhs: Box::new(Expr::Integer(0)),
                    op: get_operator(content.as_rule()),
                    rhs: Box::new(build_ast_from_expr(inner.next().unwrap(), env)?),
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
                        Some(info) => Ok(Expr::Var {
                            name,
                            info: info.clone(),
                        }),
                        None => Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: String::from(format!("{} is undefined!", name)),
                            },
                            content.as_span(),
                        )),
                    }
                }
                Rule::num => Ok(Expr::Integer(content.as_str().parse::<i32>().unwrap())),
                Rule::assign => build_ast_from_expr(content, env),
                Rule::funccall => {
                    let mut inner = content.into_inner();
                    let name = inner.next().unwrap();
                    assert_eq!(name.as_rule(), Rule::ident);
                    let args = if let Some(arg) = inner.next() {
                        arg.into_inner()
                            .map(|x| build_ast_from_expr(x, env))
                            .collect::<Result<_, _>>()?
                    } else {
                        vec![]
                    };
                    Ok(Expr::FunCall {
                        name: name.as_str().into(),
                        args,
                    })
                }
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
            println!("{:?}", pair.as_str());
            return Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: String::from("innerError in parsing expr"),
                },
                pair.as_span(),
            ));
        }
    }
}

fn build_ast_from_stmt(
    pair: pest::iterators::Pair<Rule>,
    env: &mut HashMap<String, ValInfo>,
) -> Result<Stmt, Error<Rule>> {
    match pair.as_rule() {
        Rule::res => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            let expr = build_ast_from_expr(content, env)?;
            Ok(Stmt::Return { expr })
        }
        Rule::ifstmt => {
            let mut inner = pair.into_inner();
            let cond = build_ast_from_expr(inner.next().unwrap(), env)?;
            let t_branch = Box::new(build_ast_from_stmt(inner.next().unwrap(), env)?);
            let f_branch = match inner.next() {
                Some(e) => Some(Box::new(build_ast_from_stmt(e, env)?)),
                None => None,
            };
            Ok(Stmt::If {
                cond,
                t_branch,
                f_branch,
            })
        }
        Rule::whilestmt => {
            let mut inner = pair.into_inner();
            let cond = build_ast_from_expr(inner.next().unwrap(), env)?;
            let content = Box::new(build_ast_from_stmt(inner.next().unwrap(), env)?);
            Ok(Stmt::While { cond, content })
        }
        Rule::forstmt => {
            let mut inner = pair.into_inner();
            let forcond = inner.next().unwrap();
            assert_eq!(forcond.as_rule(), Rule::forcond);
            let mut forcond = forcond.into_inner();
            let init = {
                let tmp = forcond.next().unwrap();
                if tmp.as_rule() == Rule::forsep {
                    None
                } else {
                    assert_eq!(forcond.next().unwrap().as_rule(), Rule::forsep);
                    Some(build_ast_from_expr(tmp, env)?)
                }
            };
            let cond = {
                let tmp = forcond.next().unwrap();
                if tmp.as_rule() == Rule::forsep {
                    None
                } else {
                    assert_eq!(forcond.next().unwrap().as_rule(), Rule::forsep);
                    Some(build_ast_from_expr(tmp, env)?)
                }
            };
            let tail = if let Some(tmp) = forcond.next() {
                Some(build_ast_from_expr(tmp, env)?)
            } else {
                None
            };
            let tmp = inner.next().unwrap();
            let content = Box::new(build_ast_from_stmt(tmp, env)?);
            Ok(Stmt::For {
                init,
                cond,
                tail,
                content,
            })
        }
        Rule::block => Ok(Stmt::Block(
            pair.into_inner()
                .into_iter()
                .map(|x| build_ast_from_stmt(x, env))
                .collect::<Result<_, _>>()?,
        )),
        Rule::expr => {
            let mut inner = pair.into_inner();
            let content = inner.next().unwrap();
            let expr = build_ast_from_expr(content, env)?;
            Ok(Stmt::Calc { content: expr })
        }
        Rule::declare => {
            let mut inner = pair.into_inner();
            let type_name = inner.next().unwrap();
            let var_name = inner.next().unwrap().as_str();
            let offset = (env.len() + 1) * 8;
            let info = ValInfo::new(offset, build_ast_from_typename(type_name)?);
            env.insert(String::from(var_name), info);
            Ok(Stmt::Declare)
        }
        _ => {
            return Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: String::from("innerError in parsing stmt"),
                },
                pair.as_span(),
            ));
        }
    }
}

fn biuld_ast_from_funcdef(pair: pest::iterators::Pair<Rule>) -> Result<FuncDef, Error<Rule>> {
    let mut inner = pair.into_inner();
    let res_type = build_ast_from_typename(inner.next().unwrap())?;
    let name = inner.next().unwrap();
    assert_eq!(name.as_rule(), Rule::ident);
    let name: String = name.as_str().into();
    let mut tmp = inner.next().unwrap();
    let args = if tmp.as_rule() == Rule::funcindets {
        let mut a = tmp.into_inner();
        let mut info = vec![];
        while let Some(type_name) = a.next() {
            let var_name = a.next().unwrap();
            info.push((
                build_ast_from_typename(type_name)?,
                String::from(var_name.as_str()),
            ));
        }
        tmp = inner.next().unwrap();
        info
    } else {
        vec![]
    };
    assert_eq!(tmp.as_rule(), Rule::funcbody);
    let mut env = HashMap::new();
    for i in &args {
        let offset = (env.len() + 1) * 8;
        env.insert(i.1.clone(), ValInfo::new(offset, i.0.clone()));
    }
    let body = tmp
        .into_inner()
        .into_iter()
        .map(|x| build_ast_from_stmt(x, &mut env))
        .collect::<Result<_, _>>()?;
    // 16の倍数にアラインメントする
    let mut local_area = env.len();
    if local_area % 2 == 1 {
        local_area += 1;
    }
    local_area *= 8;
    Ok(FuncDef::new(name, res_type, args, body, local_area))
}

fn build_ast_from_typename(pair: pest::iterators::Pair<Rule>) -> Result<Type, Error<Rule>> {
    assert!(pair.as_rule() == Rule::typename);
    let mut inner = pair.into_inner();
    let base = inner.next().unwrap();
    assert!(base.as_rule() == Rule::typeident);
    let mut base = get_type(base.as_str());
    while inner.next().is_some() {
        base = Type::Ptr(Box::new(base));   
    }
    Ok(base)
}

pub fn source_to_ast(source: &str) -> Result<Vec<FuncDef>, Error<Rule>> {
    let pair = CalcParser::parse(Rule::main, source)?.next().unwrap();
    let mut pair = pair.into_inner().into_iter().collect::<Vec<_>>();
    pair.pop();
    let v = pair
        .into_iter()
        .map(|x| biuld_ast_from_funcdef(x))
        .collect::<Vec<_>>();
    v.into_iter().collect::<Result<_, _>>()
}
