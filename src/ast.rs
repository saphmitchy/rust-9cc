use crate::binary::{Operation, RegisterOrNum};

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
    FunCall {
        name: String,
        args: Vec<Expr>,
    },
}

pub enum Stmt {
    Calc {
        content: Expr,
    },
    Return {
        expr: Expr,
    },
    If {
        cond: Expr,
        t_branch: Box<Stmt>,
        f_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        content: Box<Stmt>,
    },
    For {
        init: Option<Expr>,
        cond: Option<Expr>,
        tail: Option<Expr>,
        content: Box<Stmt>,
    },
    Block(Vec<Stmt>),
}

pub struct FuncDef {
    name: String,
    args: Vec<String>,
    body: Vec<Stmt>,
    local_area: usize,
}

impl FuncDef {
    pub fn new(name: String, args: Vec<String>, body: Vec<Stmt>, local_area: usize) -> FuncDef {
        FuncDef {
            name,
            args,
            body,
            local_area,
        }
    }
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

pub trait GenAssembly {
    fn to_assembly(&self, out: &mut Vec<Operation>, label_counter: &mut usize) -> ();
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
}

impl GenAssembly for Expr {
    fn to_assembly(&self, out: &mut Vec<Operation>, label_counter: &mut usize) -> () {
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
                    rhs.to_assembly(out, label_counter);
                    out.push(Pop(Rdi));
                    out.push(Pop(Rax));
                    out.push(Store(Rax, Rdi));
                    out.push(Push(Rdi));
                    return;
                }
                lhs.to_assembly(out, label_counter);
                rhs.to_assembly(out, label_counter);
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
            Expr::FunCall { name, args } => {
                for i in args {
                    i.to_assembly(out, label_counter);
                }
                let args_num = args.len();
                assert!(args_num <= 6, "関数の引数は6つまででです。");
                out.push(Mov(Rax, Num(args_num as i32)));
                let arg_regi = vec![Rdi, Rsi, Rdx, Rcx, R8, R9];
                for i in (0..args_num).rev() {
                    out.push(Pop(arg_regi[i].clone()));
                }
                out.push(Call(name.clone()));
                out.push(Push(Rax));
            }
        }
    }
}

impl GenAssembly for Stmt {
    fn to_assembly(&self, out: &mut Vec<Operation>, label_counter: &mut usize) {
        use Operation::*;
        use RegisterOrNum::*;
        match self {
            Stmt::Return { expr } => {
                expr.to_assembly(out, label_counter);
                out.push(Pop(Rax));
                out.push(Mov(Rsp, Rbp));
                out.push(Pop(Rbp));
                out.push(Ret);
            }
            Stmt::If {
                cond,
                t_branch,
                f_branch,
            } => {
                cond.to_assembly(out, label_counter);
                out.push(Pop(Rax));
                out.push(Cmp(Rax, Num(0)));
                let crr_label = *label_counter + 1;
                *label_counter = *label_counter + 1;
                if let Some(f_branch) = &*f_branch {
                    out.push(Je("else", crr_label));
                    t_branch.to_assembly(out, label_counter);
                    out.push(Jmp("end", crr_label));
                    out.push(Label("else", crr_label));
                    f_branch.to_assembly(out, label_counter);
                    out.push(Label("end", crr_label));
                } else {
                    out.push(Je("end", crr_label));
                    t_branch.to_assembly(out, label_counter);
                    out.push(Label("end", crr_label));
                }
            }
            Stmt::Block(v) => {
                for i in v {
                    i.to_assembly(out, label_counter);
                }
            }
            Stmt::While { cond, content } => {
                let crr_label = *label_counter + 1;
                *label_counter = *label_counter + 1;
                out.push(Label("begin", crr_label));
                cond.to_assembly(out, label_counter);
                out.push(Pop(Rax));
                out.push(Cmp(Rax, Num(0)));
                out.push(Je("end", crr_label));
                content.to_assembly(out, label_counter);
                out.push(Jmp("begin", crr_label));
                out.push(Label("end", crr_label));
            }
            Stmt::For {
                init,
                cond,
                tail,
                content,
            } => {
                let crr_label = *label_counter + 1;
                *label_counter = *label_counter + 1;
                if let Some(init) = init {
                    init.to_assembly(out, label_counter);
                }
                out.push(Label("begin", crr_label));
                if let Some(cond) = cond {
                    cond.to_assembly(out, label_counter);
                    out.push(Pop(Rax));
                    out.push(Cmp(Rax, Num(0)));
                    out.push(Je("end", crr_label));
                }
                content.to_assembly(out, label_counter);
                if let Some(tail) = tail {
                    tail.to_assembly(out, label_counter);
                }
                out.push(Jmp("begin", crr_label));
                out.push(Label("end", crr_label));
            }
            Stmt::Calc { content } => {
                content.to_assembly(out, label_counter);
                out.push(Pop(Rax));
            }
        }
    }
}

impl GenAssembly for FuncDef {
    fn to_assembly(&self, out: &mut Vec<Operation>, label_counter: &mut usize) -> () {
        use crate::binary::Operation::*;
        use crate::binary::RegisterOrNum::*;
        out.push(Func(self.name.clone()));
        out.push(Push(Rbp));
        out.push(Mov(Rbp, Rsp));
        out.push(Sub(Rsp, Num(self.local_area as i32)));
        // 関数の引数をスタックにコピーする
        let arg_regi = vec![Rdi, Rsi, Rdx, Rcx, R8, R9];
        out.push(Mov(Rax, Rbp));
        let args_num = self.args.len();
        assert!(args_num <= 6, "関数の引数は6つまででです。");
        for i in 0..args_num {
            out.push(Sub(Rax, Num(8)));
            out.push(Store(Rax, arg_regi[i].clone()));
        }
        for i in &self.body {
            i.to_assembly(out, label_counter);
        }
        out.push(Mov(Rsp, Rbp));
        out.push(Pop(Rbp));
        out.push(Ret);
    }
}
