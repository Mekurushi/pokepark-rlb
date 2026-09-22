#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Integer(IntegerKind),
    Float(FloatKind),
    String,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatKind {
    F32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldConstraint {
    None,
    IntegerRange { min: u32, max: u32 },
    TableIndex { table: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: FieldKind,
    pub constraint: FieldConstraint,
}
