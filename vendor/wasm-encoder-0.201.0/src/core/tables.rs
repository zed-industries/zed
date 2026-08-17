use crate::{encode_section, ConstExpr, Encode, RefType, Section, SectionId};

/// An encoder for the table section.
///
/// Table sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Module, TableSection, TableType, RefType};
///
/// let mut tables = TableSection::new();
/// tables.table(TableType {
///     element_type: RefType::FUNCREF,
///     minimum: 128,
///     maximum: None,
/// });
///
/// let mut module = Module::new();
/// module.section(&tables);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct TableSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl TableSection {
    /// Construct a new table section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of tables in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a table.
    pub fn table(&mut self, table_type: TableType) -> &mut Self {
        table_type.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Define a table with an explicit initialization expression.
    ///
    /// Note that this is part of the function-references proposal.
    pub fn table_with_init(&mut self, table_type: TableType, init: &ConstExpr) -> &mut Self {
        self.bytes.push(0x40);
        self.bytes.push(0x00);
        table_type.encode(&mut self.bytes);
        init.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for TableSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for TableSection {
    fn id(&self) -> u8 {
        SectionId::Table.into()
    }
}

/// A table's type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableType {
    /// The table's element type.
    pub element_type: RefType,
    /// Minimum size, in elements, of this table
    pub minimum: u32,
    /// Maximum size, in elements, of this table
    pub maximum: Option<u32>,
}

impl Encode for TableType {
    fn encode(&self, sink: &mut Vec<u8>) {
        let mut flags = 0;
        if self.maximum.is_some() {
            flags |= 0b001;
        }

        self.element_type.encode(sink);
        sink.push(flags);
        self.minimum.encode(sink);

        if let Some(max) = self.maximum {
            max.encode(sink);
        }
    }
}

#[cfg(feature = "wasmparser")]
impl TryFrom<wasmparser::TableType> for TableType {
    type Error = ();
    fn try_from(table_ty: wasmparser::TableType) -> Result<Self, Self::Error> {
        Ok(TableType {
            element_type: table_ty.element_type.try_into()?,
            minimum: table_ty.initial,
            maximum: table_ty.maximum,
        })
    }
}
