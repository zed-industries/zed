use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Row {
    file: *const File,
    index: usize,
}

unsafe impl Send for Row {}
unsafe impl Sync for Row {}

impl Row {
    pub(crate) fn new(file: &'static File, index: usize) -> Self {
        Self { file, index }
    }
}

pub trait AsRow: Copy {
    const TABLE: usize;
    fn to_row(&self) -> Row;
    fn from_row(row: Row) -> Self;

    fn file(&self) -> &'static File {
        unsafe { &*self.to_row().file }
    }

    fn reader(&self) -> &'static Reader {
        // Safety: At this point the File is already pointing to a valid Reader.
        unsafe { &*self.file().reader }
    }

    fn index(&self) -> usize {
        self.to_row().index
    }

    fn usize(&self, column: usize) -> usize {
        self.file().usize(self.index(), Self::TABLE, column)
    }

    fn str(&self, column: usize) -> &'static str {
        self.file().str(self.index(), Self::TABLE, column)
    }

    fn row(&self, column: usize) -> Row {
        Row::new(self.file(), self.usize(column) - 1)
    }

    fn decode<T: Decode>(&self, column: usize) -> T {
        T::decode(self.file(), self.usize(column))
    }

    fn blob(&self, column: usize) -> Blob {
        self.file().blob(self.index(), Self::TABLE, column)
    }

    fn list<R: AsRow>(&self, column: usize) -> RowIterator<R> {
        self.file().list(self.index(), Self::TABLE, column)
    }
}

pub struct RowIterator<R: AsRow> {
    file: &'static File,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<R: AsRow> RowIterator<R> {
    pub(crate) fn new(file: &'static File, rows: std::ops::Range<usize>) -> Self {
        Self {
            file,
            rows,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<R: AsRow> Iterator for RowIterator<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows
            .next()
            .map(|row| R::from_row(Row::new(self.file, row)))
    }
}

pub trait HasAttributes {
    fn attributes(&self) -> RowIterator<Attribute>;
    fn find_attribute(&self, name: &str) -> Option<Attribute>;
    fn has_attribute(&self, name: &str) -> bool;
    fn guid_attribute(&self) -> Option<GUID>;
    fn arches(&self) -> i32;
}

impl<R: AsRow + Into<HasAttribute>> HasAttributes for R {
    fn attributes(&self) -> RowIterator<Attribute> {
        self.file()
            .equal_range(0, Into::<HasAttribute>::into(*self).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute> {
        self.attributes().find(|attribute| attribute.name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }

    fn guid_attribute(&self) -> Option<GUID> {
        self.find_attribute("GuidAttribute").map(|attribute| {
            fn unwrap_u32(value: &Value) -> u32 {
                match value {
                    Value::U32(value) => *value,
                    _ => panic!(),
                }
            }
            fn unwrap_u16(value: &Value) -> u16 {
                match value {
                    Value::U16(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }
            fn unwrap_u8(value: &Value) -> u8 {
                match value {
                    Value::U8(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }

            let args = attribute.args();

            GUID(
                unwrap_u32(&args[0].1),
                unwrap_u16(&args[1].1),
                unwrap_u16(&args[2].1),
                unwrap_u8(&args[3].1),
                unwrap_u8(&args[4].1),
                unwrap_u8(&args[5].1),
                unwrap_u8(&args[6].1),
                unwrap_u8(&args[7].1),
                unwrap_u8(&args[8].1),
                unwrap_u8(&args[9].1),
                unwrap_u8(&args[10].1),
            )
        })
    }

    fn arches(&self) -> i32 {
        let mut arches = 0;

        if let Some(attribute) = self.find_attribute("SupportedArchitectureAttribute") {
            if let Some((_, Value::I32(value))) = attribute.args().first() {
                arches = *value;
            }
        }

        arches
    }
}
