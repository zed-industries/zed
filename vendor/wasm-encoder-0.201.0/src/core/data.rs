use crate::{encode_section, encoding_size, ConstExpr, Encode, Section, SectionId};

/// An encoder for the data section.
///
/// Data sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{
///     ConstExpr, DataSection, Instruction, MemorySection, MemoryType,
///     Module,
/// };
///
/// let mut memory = MemorySection::new();
/// memory.memory(MemoryType {
///     minimum: 1,
///     maximum: None,
///     memory64: false,
///     shared: false,
/// });
///
/// let mut data = DataSection::new();
/// let memory_index = 0;
/// let offset = ConstExpr::i32_const(42);
/// let segment_data = b"hello";
/// data.active(memory_index, &offset, segment_data.iter().copied());
///
/// let mut module = Module::new();
/// module
///     .section(&memory)
///     .section(&data);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct DataSection {
    bytes: Vec<u8>,
    num_added: u32,
}

/// A segment in the data section.
#[derive(Clone, Debug)]
pub struct DataSegment<'a, D> {
    /// This data segment's mode.
    pub mode: DataSegmentMode<'a>,
    /// This data segment's data.
    pub data: D,
}

/// A data segment's mode.
#[derive(Clone, Debug)]
pub enum DataSegmentMode<'a> {
    /// An active data segment.
    Active {
        /// The memory this segment applies to.
        memory_index: u32,
        /// The offset where this segment's data is initialized at.
        offset: &'a ConstExpr,
    },
    /// A passive data segment.
    ///
    /// Passive data segments are part of the bulk memory proposal.
    Passive,
}

impl DataSection {
    /// Create a new data section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of data segments in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a data segment.
    pub fn segment<D>(&mut self, segment: DataSegment<D>) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        match segment.mode {
            DataSegmentMode::Passive => {
                self.bytes.push(0x01);
            }
            DataSegmentMode::Active {
                memory_index: 0,
                offset,
            } => {
                self.bytes.push(0x00);
                offset.encode(&mut self.bytes);
            }
            DataSegmentMode::Active {
                memory_index,
                offset,
            } => {
                self.bytes.push(0x02);
                memory_index.encode(&mut self.bytes);
                offset.encode(&mut self.bytes);
            }
        }

        let data = segment.data.into_iter();
        data.len().encode(&mut self.bytes);
        self.bytes.extend(data);

        self.num_added += 1;
        self
    }

    /// Define an active data segment.
    pub fn active<D>(&mut self, memory_index: u32, offset: &ConstExpr, data: D) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        self.segment(DataSegment {
            mode: DataSegmentMode::Active {
                memory_index,
                offset,
            },
            data,
        })
    }

    /// Define a passive data segment.
    ///
    /// Passive data segments are part of the bulk memory proposal.
    pub fn passive<D>(&mut self, data: D) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        self.segment(DataSegment {
            mode: DataSegmentMode::Passive,
            data,
        })
    }

    /// Copy an already-encoded data segment into this data section.
    pub fn raw(&mut self, already_encoded_data_segment: &[u8]) -> &mut Self {
        self.bytes.extend_from_slice(already_encoded_data_segment);
        self.num_added += 1;
        self
    }
}

impl Encode for DataSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for DataSection {
    fn id(&self) -> u8 {
        SectionId::Data.into()
    }
}

/// An encoder for the data count section.
#[derive(Clone, Copy, Debug)]
pub struct DataCountSection {
    /// The number of segments in the data section.
    pub count: u32,
}

impl Encode for DataCountSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encoding_size(self.count).encode(sink);
        self.count.encode(sink);
    }
}

impl Section for DataCountSection {
    fn id(&self) -> u8 {
        SectionId::DataCount.into()
    }
}
