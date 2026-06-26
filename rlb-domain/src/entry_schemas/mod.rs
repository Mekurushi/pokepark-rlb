pub mod script_list;
pub mod single_pointer;

use crate::Value;
use crate::rlb_file::StringId;
use rlb_error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: &'static str,
}

pub trait TableEntry: Sized + std::fmt::Debug {
    const SIZE: usize;
    const TYPE_NAME: &'static str;

    fn fields(&self) -> &[FieldDescriptor];
    fn is_terminator(&self) -> bool;

    fn get(&self, field: &str) -> Option<Value>;

    fn set(&mut self, field: &str, value: Value) -> Result<()>;

    fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool;
}
#[macro_export]
macro_rules! impl_table_entry_wrapper {
    (
        struct $wrapper:ident($inner:ty);

        fields = $fields:ident;
    ) => {
        impl_table_entry_wrapper! {
            struct $wrapper($inner);

            type_name = stringify!($wrapper);
            fields = $fields;
        }
    };

    (
        struct $wrapper:ident($inner:ty);

        type_name = $type_name:expr;
        fields = $fields:ident;
    ) => {
        #[derive(Debug, Clone)]
        pub struct $wrapper(pub $inner);

        impl TableEntry for $wrapper {
            const SIZE: usize = <$inner>::SIZE;
            const TYPE_NAME: &'static str = $type_name;

            fn fields(&self) -> &[FieldDescriptor] {
                $fields
            }

            fn is_terminator(&self) -> bool {
                self.0.is_terminator()
            }

            fn get(&self, field: &str) -> Option<Value> {
                self.0.get(field)
            }

            fn set(&mut self, field: &str, value: Value) -> Result<()> {
                self.0.set(field, value)
            }

            fn read<R, E>(
                data: &[u8],
                base_offset: usize,
                resolve_string: &mut R,
                is_relocated: &mut E,
            ) -> Result<Self>
            where
                R: FnMut(u32) -> Result<StringId>,
                E: FnMut(u32) -> bool,
            {
                Ok(Self(<$inner>::read(
                    data,
                    base_offset,
                    resolve_string,
                    is_relocated,
                )?))
            }
        }
    };
}
