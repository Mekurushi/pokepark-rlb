use crate::rlb_file::StringId;
use rlb_error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    String(Option<StringId>),
    Boolean(bool),
}
impl Value {
    pub(crate) fn as_integer(self) -> Result<u32> {
        match self {
            Value::Integer(v) => Ok(v),
            _ => Err(Error::Validation(
                "expected Integer value in write path; schema mismatch".into(),
            )),
        }
    }
    pub(crate) fn as_bool(self) -> Result<bool> {
        match self {
            Value::Boolean(v) => Ok(v),
            _ => Err(Error::Validation(
                "expected Boolean value in write path; schema mismatch".into(),
            )),
        }
    }
}
