#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Ptr(Box<Type>),
}

pub fn get_type(name: &str) -> Type {
    if name == "int" {
        return Type::Int;
    } else {
        panic!("type name `{}` is not exit!", name);
    }
}
