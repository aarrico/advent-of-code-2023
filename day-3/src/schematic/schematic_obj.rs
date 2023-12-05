use std::fmt;
use std::fmt::Formatter;
use std::mem::discriminant;

#[derive(Debug, Copy, Clone)]
pub enum SchematicObj {
    Number(char),
    Symbol(char),
    Period(char),
}

impl SchematicObj {
    pub fn is_number(&self) -> bool {
        Self::is_same_variant(&self, &Self::Number('0'))
    }

    pub fn is_symbol(&self) -> bool {
        self.is_same_variant(&Self::Symbol('*'))
    }

    pub fn is_period(&self) -> bool {
        self.is_same_variant(&Self::Period('.'))
    }

    pub fn determine_type(c: char) -> Self {
        if let Some(num) = c.to_digit(10) {
            return Self::Number(c);
        } else if c != '.' {
            return Self::Symbol(c);
        }

        Self::Period('.')
    }

    pub fn get_value(self) -> char {
        match self {
            SchematicObj::Number(number) => number,
            SchematicObj::Symbol(symbol) => symbol,
            SchematicObj::Period(period) => period
        }
    }

    fn is_same_variant(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(&other)
    }
}

impl fmt::Display for SchematicObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SchematicObj::Number(num) => { write!(f, "{}", num) }
            SchematicObj::Symbol(sym) => { write!(f, "{}", sym) }
            SchematicObj::Period(period) => write!(f, ".")
        }
    }
}