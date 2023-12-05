use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum MatrixObj {
    Number(u8),
    Symbol(char),
    Period,
}

impl MatrixObj {
    pub fn is_symbol(&self) -> bool {
        if let MatrixObj::Symbol { .. } = self {
            return true;
        }

        return false;
    }

    pub fn determine_type(c: char) -> Self {
        if c.is_ascii_digit() {
            return Self::Number(c as u8);
        } else if c != '.' {
            return Self::Symbol(c);
        }

        return Self::Period;
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