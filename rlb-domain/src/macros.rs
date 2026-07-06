macro_rules! declare_tables {
    (
        $(
            $schema:ident {
                body: $body:ty,

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
        $schema($body),
    )*

    Unknown, // TODO: Unknown should hold/display raw bytes
}

impl TableKind {
    pub fn discover<R, E>(
        name: &str,
        data: &[u8],
        offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> rlb_error::Result<Self>
    where
        R: FnMut(u32) -> rlb_error::Result<crate::rlb_file::StringId>,
        E: FnMut(u32) -> bool,
    {
        match name {
            $(
                $(
                    $name => {
                        return Ok(Self::$schema(
                            <$body as crate::table::body::TableBody>::discover(
                                data,
                                offset,
                                resolve_string,
                                is_relocated,
                            )?
                        ));
                    }
                )*
            )*

            _ => Ok(Self::Unknown),
        }
    }

    pub fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &crate::string_pool::SerializedStringPoolContext<crate::rlb_file::StringId>,
        relocations: &mut Vec<u32>,
    ) -> rlb_error::Result<()> {
        match self {
            $(
                Self::$schema(body) => crate::table::body::TableBody::serialize(
                    body, out, base_offset, strings, relocations,
                ),
            )*
            Self::Unknown => Ok(()),
        }
    }

    pub(crate) fn entry_count(&self) -> usize {
        match self {
            $(Self::$schema(body) => crate::table::body::TableBody::entry_count(body),)*
            Self::Unknown => 0,
        }
    }

    pub(crate) fn field_descriptors(&self) -> &'static [crate::FieldDescriptor] {
        match self {
            $(Self::$schema(body) => crate::table::body::TableBody::fields(body),)*
            Self::Unknown => &[],
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<crate::Value> {
        match self {
            $(Self::$schema(body) => crate::table::body::TableBody::get_field(body, index, field),)*
            Self::Unknown => None,
        }
    }

    pub(crate) fn set_field(
        &mut self,
        index: usize,
        field: &str,
        value: crate::Value,
    ) -> rlb_error::Result<()> {
        match self {
            $(Self::$schema(body) => crate::table::body::TableBody::set_field(body, index, field, value),)*
            Self::Unknown => Err(rlb_error::Error::Validation("cannot mutate an Unknown table".into())),
        }
    }
}

    };
}

pub(super) use declare_tables;
