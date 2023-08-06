use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub enum RegisterOrNum {
    Rdi,
    Rdx,
    Rax,
    Rbp,
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
    Mov(RegisterOrNum, RegisterOrNum),
    Load(RegisterOrNum, RegisterOrNum),
    Store(RegisterOrNum, RegisterOrNum),
    Movzb(RegisterOrNum, RegisterOrNum),
}

impl fmt::Display for RegisterOrNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rdi => write!(f, "rdi"),
            Self::Rdx => write!(f, "rdx"),
            Self::Rax => write!(f, "rax"),
            Self::Rbp => write!(f, "rbp"),
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
            Self::Mov(r1, r2) => write!(f, "  mov {}, {}", r1, r2),
            Self::Load(r1, r2) => write!(f, "  mov {}, [{}]", r1, r2),
            Self::Store(r1, r2) => write!(f, "  mov [{}], {}", r1, r2),
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
    file.write_all(b"  push rbp\n")?;
    file.write_all(b"  mov rbp, rsp\n")?;
    file.write_all(b"  sub rsp, 208\n")?;
    for i in oprations {
        write!(file, "{}\n", i)?;
    }
    file.write_all(b"  mov rsp, rbp\n")?;
    file.write_all(b"  pop rbp\n")?;
    file.write_all(b"  ret\n")
}
