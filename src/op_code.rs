use core::fmt;
use std::fmt::{Display, Formatter};

pub enum OpCode {
    Constant,
    Return,
    Unknown,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::Constant => 0,
            OpCode::Return => 1,
            OpCode::Unknown => 2,
        }
    }
}

impl From<&u8> for OpCode {
    fn from(value: &u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => OpCode::Unknown,
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        OpCode::from(&value)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant => write!(f, "OP_CONSTANT"),
            OpCode::Return => write!(f, "OP_RETURN"),
            OpCode::Unknown => write!(f, "OP_UNKNOWN"),
        }
    }
}
