use std::fmt;
use std::fmt::Formatter;
use std::mem::discriminant;

#[derive(Debug)]
pub enum MatrixObj {
    Number(u32),
    Symbol(char),
    Period,
}

impl MatrixObj {
    pub fn is_number(&self) -> bool {
        Self::is_same_variant(&self, &Self::Number(0))
    }

    pub fn is_symbol(&self) -> bool {
        self.is_same_variant(&Self::Symbol('*'))
    }

    pub fn determine_type(c: char) -> Self {
        if let Some(num) = c.to_digit(10) {
            return Self::Number(num);
        } else if c != '.' {
            return Self::Symbol(c);
        }

        Self::Period
    }

    fn is_same_variant(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(&other)
    }
}

impl fmt::Display for MatrixObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MatrixObj::Number(num) => { write!(f, "{}", num) }
            MatrixObj::Symbol(sym) => { write!(f, "{}", sym) }
            MatrixObj::Period => write!(f, ".")
        }
    }
}