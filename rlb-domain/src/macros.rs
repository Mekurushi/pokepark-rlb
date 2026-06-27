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
            TableView<$entry>
        ),
    )*

    Unknown,
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
                    TableView::discover(
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


}


};

}

pub(super) use declare_tables;
