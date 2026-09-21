use rlb_error::{Error, Result};

//TODO: check if Integer should be split
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(u32),
    Float(f32),
    String(Option<String>),
    Boolean(bool),
}
impl Value {
    pub(crate) fn as_integer(&self) -> Result<u32> {
        match self {
            Value::Integer(v) => Ok(*v),
            _ => Err(Error::Validation(
                "expected Integer value in write path; schema mismatch".into(),
            )),
        }
    }
    pub(crate) fn as_bool(&self) -> Result<bool> {
        match self {
            Value::Boolean(v) => Ok(*v),
            _ => Err(Error::Validation(
                "expected Boolean value in write path; schema mismatch".into(),
            )),
        }
    }

    pub(crate) fn as_float(&self) -> Result<f32> {
        match self {
            Value::Float(v) => Ok(*v),
            _ => Err(Error::Validation(
                "expected Float value in write path; schema mismatch".into(),
            )),
        }
    }
}
