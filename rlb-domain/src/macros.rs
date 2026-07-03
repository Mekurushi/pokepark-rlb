macro_rules! declare_tables {

(
    $(
        $schema:ident {
            entry: $entry:path,

            tables: [
                $($name:literal),* $(,)?
            ]
        }
    )*
)

=> {


#[derive(Debug, Clone)]
pub enum TableKind {

    $(
        $schema(
            EntryList<$entry>
        ),
    )*

    Unknown, // TODO: Unknown should hold/display raw bytes
}



impl TableKind {


pub fn discover<R,E>(
    name: &str,
    data: &[u8],
    offset: usize,
    resolve_string: &mut R,
    is_relocated: &mut E,
)
-> Result<Self>

where
    R: FnMut(u32) -> Result<StringId>,
    E: FnMut(u32) -> bool,
{

match name {


$(
    $(
        $name => {

            return Ok(
                Self::$schema(
                    EntryList::discover(
                        data,
                        offset,
                        resolve_string,
                        is_relocated,
                    )?
                )
            );

        }
    )*
)*


_ => Ok(Self::Unknown)


}

}
    pub fn serialize(
    &self,
    out: &mut Vec<u8>,
    base_offset: usize,
    strings: &crate::string_pool::SerializedStringPoolContext<StringId>,
    relocations: &mut Vec<u32>,
) -> Result<()> {
    match self {
        $(
            Self::$schema(view) => view.serialize(out, base_offset, strings, relocations),
        )*
        Self::Unknown => Ok(()),
    }
}

    pub(crate) fn entry_count(&self) -> usize {
        match self {
            $(Self::$schema(view) => view.entries.len(),)*
            Self::Unknown => 0,
        }
    }
    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        match self {
            $(Self::$schema(view) => view.fields(),)*
            Self::Unknown => &[],
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        match self {
            $(Self::$schema(view) => view.entries.get(index)?.get(field),)*
            Self::Unknown => None,
        }
    }



}


};

}

pub(super) use declare_tables;
