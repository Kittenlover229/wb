#[derive(Debug, Clone)]
pub enum Type {
    Variable(u128),
    Integer,
    Bool,
}

pub trait Typed {
    fn is_complete(&self) -> bool;
}

impl Typed for Type {
    fn is_complete(&self) -> bool {
        match self {
            Self::Variable(_) => false,
            Self::Integer | Self::Bool => true,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variable(l0), Self::Variable(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Variable(0)
    }
}
