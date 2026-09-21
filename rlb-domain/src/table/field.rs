#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Integer,
    String,
    Boolean,
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
