use std::{
    io::{self, Read, Seek, SeekFrom},
    mem::size_of,
};

use crate::{
    date::{Date, InfiniteOrNanDate},
    error::{Error, ErrorKind},
    stream::{Event, OwnedEvent},
    u64_to_usize, Uid,
};

struct StackItem {
    object_ref: u64,
    child_object_refs: Vec<u64>,
    ty: StackType,
}

enum StackType {
    Array,
    Dict,
}

// https://opensource.apple.com/source/CF/CF-550/CFBinaryPList.c
// https://hg.python.org/cpython/file/3.4/Lib/plistlib.py
pub struct BinaryReader<R> {
    stack: Vec<StackItem>,
    object_offsets: Vec<u64>,
    object_on_stack: Vec<bool>,
    reader: PosReader<R>,
    ref_size: u8,
    root_object: u64,
    trailer_start_offset: u64,
}

struct PosReader<R> {
    reader: R,
    pos: u64,
}

impl<R: Read + Seek> PosReader<R> {
    fn read_all(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.read_exact(buf)
            .map_err(|err| ErrorKind::Io(err).with_byte_offset(self.pos))?;
        Ok(())
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        self.pos = self
            .reader
            .seek(pos)
            .map_err(|err| ErrorKind::Io(err).with_byte_offset(self.pos))?;
        Ok(self.pos)
    }
}

impl<R: Read> Read for PosReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.pos
            .checked_add(count as u64)
            .expect("file cannot be larger than `u64::MAX` bytes");
        Ok(count)
    }
}

impl<R: Read + Seek> BinaryReader<R> {
    pub fn new(reader: R) -> BinaryReader<R> {
        BinaryReader {
            stack: Vec::new(),
            object_offsets: Vec::new(),
            object_on_stack: Vec::new(),
            reader: PosReader { reader, pos: 0 },
            ref_size: 0,
            root_object: 0,
            trailer_start_offset: 0,
        }
    }

    fn allocate_vec<T>(&self, len: u64, size: usize) -> Result<Vec<T>, Error> {
        // Check we are not reading past the start of the plist trailer
        let inner = |len: u64, size: usize| {
            let byte_len = len.checked_mul(size as u64)?;
            let end_offset = self.reader.pos.checked_add(byte_len)?;
            if end_offset <= self.trailer_start_offset {
                Some(())
            } else {
                None
            }
        };
        inner(len, size).ok_or_else(|| self.with_pos(ErrorKind::ObjectOffsetTooLarge))?;

        Ok(Vec::with_capacity(len as usize))
    }

    fn read_trailer(&mut self) -> Result<(), Error> {
        self.reader.seek(SeekFrom::Start(0))?;
        let mut magic = [0; 8];
        self.reader.read_all(&mut magic)?;
        if &magic != b"bplist00" {
            return Err(self.with_pos(ErrorKind::InvalidMagic));
        }

        self.trailer_start_offset = self.reader.seek(SeekFrom::End(-32))?;

        // Trailer starts with 6 bytes of padding
        let mut zeros = [0; 6];
        self.reader.read_all(&mut zeros)?;

        // At least one encoder can use 24-bit integers in the object offset table.
        // We never write 24-bit integers as not all decoders support them.
        let offset_size = self.read_u8()?;
        match offset_size {
            1 | 2 | 3 | 4 | 8 => (),
            _ => return Err(self.with_pos(ErrorKind::InvalidTrailerObjectOffsetSize)),
        }

        self.ref_size = self.read_u8()?;
        match self.ref_size {
            1 | 2 | 4 | 8 => (),
            _ => return Err(self.with_pos(ErrorKind::InvalidTrailerObjectReferenceSize)),
        }

        let num_objects = self.read_be_u64()?;
        self.root_object = self.read_be_u64()?;
        let offset_table_offset = self.read_be_u64()?;

        // Read offset table
        self.reader.seek(SeekFrom::Start(offset_table_offset))?;
        self.object_offsets = self.read_ints(num_objects, offset_size)?;
        self.object_on_stack = vec![false; self.object_offsets.len()];

        Ok(())
    }

    /// Reads a list of `len` big-endian integers of `size` bytes from the reader.
    fn read_ints(&mut self, len: u64, size: u8) -> Result<Vec<u64>, Error> {
        let mut ints = self.allocate_vec(len, size as usize)?;
        for _ in 0..len {
            match size {
                1 => ints.push(self.read_u8()?.into()),
                2 => ints.push(self.read_be_u16()?.into()),
                // At least one encoder can use 24-bit integers in the object offset table.
                // We never write 24-bit integers as not all decoders support them.
                3 => ints.push(self.read_be_u24()?.into()),
                4 => ints.push(self.read_be_u32()?.into()),
                8 => ints.push(self.read_be_u64()?),
                _ => unreachable!("size is either self.ref_size or offset_size both of which are already validated")
            }
        }
        Ok(ints)
    }

    /// Reads a list of `len` offsets into the object table from the reader.
    fn read_refs(&mut self, len: u64) -> Result<Vec<u64>, Error> {
        let ref_size = self.ref_size;
        self.read_ints(len, ref_size)
    }

    /// Reads a compressed value length from the reader. `len` must contain the low 4 bits of the
    /// object token.
    fn read_object_len(&mut self, len: u8) -> Result<u64, Error> {
        if (len & 0x0f) == 0x0f {
            let len_power_of_two = self.read_u8()? & 0x03;
            Ok(match len_power_of_two {
                0 => self.read_u8()?.into(),
                1 => self.read_be_u16()?.into(),
                2 => self.read_be_u32()?.into(),
                3 => self.read_be_u64()?,
                _ => return Err(self.with_pos(ErrorKind::InvalidObjectLength)),
            })
        } else {
            Ok(len.into())
        }
    }

    /// Reads `len` bytes from the reader.
    fn read_data(&mut self, len: u64) -> Result<Vec<u8>, Error> {
        let mut data = self.allocate_vec(len, size_of::<u8>())?;
        data.resize(len as usize, 0);
        self.reader.read_all(&mut data)?;
        Ok(data)
    }

    fn seek_to_object(&mut self, object_ref: u64) -> Result<u64, Error> {
        let object_ref = u64_to_usize(object_ref)
            .ok_or_else(|| self.with_pos(ErrorKind::ObjectReferenceTooLarge))?;
        let offset = *self
            .object_offsets
            .get(object_ref)
            .ok_or_else(|| self.with_pos(ErrorKind::ObjectReferenceTooLarge))?;
        if offset >= self.trailer_start_offset {
            return Err(self.with_pos(ErrorKind::ObjectOffsetTooLarge));
        }
        self.reader.seek(SeekFrom::Start(offset))
    }

    fn push_stack_item_and_check_for_recursion(&mut self, item: StackItem) -> Result<(), Error> {
        let object_ref = u64_to_usize(item.object_ref).expect("internal consistency error");
        let is_on_stack = &mut self.object_on_stack[object_ref];
        if *is_on_stack {
            return Err(self.with_pos(ErrorKind::RecursiveObject));
        }
        *is_on_stack = true;
        self.stack.push(item);
        Ok(())
    }

    fn pop_stack_item(&mut self) -> StackItem {
        let item = self.stack.pop().expect("internal consistency error");
        let object_ref = u64_to_usize(item.object_ref).expect("internal consistency error");
        self.object_on_stack[object_ref] = false;
        item
    }

    fn read_next(&mut self) -> Result<Option<OwnedEvent>, Error> {
        let object_ref = if self.ref_size == 0 {
            // Initialise here rather than in new
            self.read_trailer()?;
            self.root_object
        } else {
            let maybe_object_ref = if let Some(stack_item) = self.stack.last_mut() {
                stack_item.child_object_refs.pop()
            } else {
                // Finished reading the plist
                return Ok(None);
            };

            if let Some(object_ref) = maybe_object_ref {
                object_ref
            } else {
                // We're at the end of an array or dict. Pop the top stack item and return.
                let stack_item = self.pop_stack_item();
                match stack_item.ty {
                    StackType::Array | StackType::Dict => return Ok(Some(Event::EndCollection)),
                }
            }
        };

        self.seek_to_object(object_ref)?;

        let token = self.read_u8()?;
        let ty = (token & 0xf0) >> 4;
        let size = token & 0x0f;

        let result = match (ty, size) {
            (0x0, 0x00) => return Err(self.with_pos(ErrorKind::NullObjectUnimplemented)),
            (0x0, 0x08) => Some(Event::Boolean(false)),
            (0x0, 0x09) => Some(Event::Boolean(true)),
            (0x0, 0x0f) => return Err(self.with_pos(ErrorKind::FillObjectUnimplemented)),
            (0x1, 0) => Some(Event::Integer(self.read_u8()?.into())),
            (0x1, 1) => Some(Event::Integer(self.read_be_u16()?.into())),
            (0x1, 2) => Some(Event::Integer(self.read_be_u32()?.into())),
            (0x1, 3) => Some(Event::Integer(self.read_be_i64()?.into())),
            (0x1, 4) => {
                let value = self.read_be_i128()?;
                if let Ok(value) = u64::try_from(value) {
                    Some(Event::Integer(value.into()))
                } else {
                    return Err(self.with_pos(ErrorKind::IntegerOutOfRange));
                }
            }
            (0x1, _) => return Err(self.with_pos(ErrorKind::UnknownObjectType(token))), // variable length int
            (0x2, 2) => Some(Event::Real(f32::from_bits(self.read_be_u32()?).into())),
            (0x2, 3) => Some(Event::Real(f64::from_bits(self.read_be_u64()?))),
            (0x2, _) => return Err(self.with_pos(ErrorKind::UnknownObjectType(token))), // odd length float
            (0x3, 3) => {
                // Date. Seconds since 1/1/2001 00:00:00.
                let secs = f64::from_bits(self.read_be_u64()?);
                let date = Date::from_seconds_since_plist_epoch(secs)
                    .map_err(|InfiniteOrNanDate| self.with_pos(ErrorKind::InfiniteOrNanDate))?;
                Some(Event::Date(date))
            }
            (0x4, n) => {
                // Data
                let len = self.read_object_len(n)?;
                Some(Event::Data(self.read_data(len)?.into()))
            }
            (0x5, n) => {
                // ASCII string
                let len = self.read_object_len(n)?;
                let raw = self.read_data(len)?;
                let string = String::from_utf8(raw)
                    .map_err(|_| self.with_pos(ErrorKind::InvalidUtf8String))?;
                Some(Event::String(string.into()))
            }
            (0x6, n) => {
                // UTF-16 string
                let len_utf16_codepoints = self.read_object_len(n)?;
                let mut raw_utf16 = self.allocate_vec(len_utf16_codepoints, size_of::<u16>())?;

                for _ in 0..len_utf16_codepoints {
                    raw_utf16.push(self.read_be_u16()?);
                }

                let string = String::from_utf16(&raw_utf16)
                    .map_err(|_| self.with_pos(ErrorKind::InvalidUtf16String))?;
                Some(Event::String(string.into()))
            }
            (0x8, n) if n < 8 => {
                // Uid
                let mut buf = [0; 8];
                // `len_bytes` is at most 8.
                let len_bytes = n as usize + 1;
                // Values are stored in big-endian so we must put the least significant bytes at
                // the end of the buffer.
                self.reader.read_all(&mut buf[8 - len_bytes..])?;
                let value = u64::from_be_bytes(buf);

                Some(Event::Uid(Uid::new(value)))
            }
            (0xa, n) => {
                // Array
                let len = self.read_object_len(n)?;
                let mut child_object_refs = self.read_refs(len)?;
                // Reverse so we can pop off the end of the stack in order
                child_object_refs.reverse();

                self.push_stack_item_and_check_for_recursion(StackItem {
                    object_ref,
                    ty: StackType::Array,
                    child_object_refs,
                })?;

                Some(Event::StartArray(Some(len)))
            }
            (0xd, n) => {
                // Dict
                let len = self.read_object_len(n)?;
                let key_refs = self.read_refs(len)?;
                let value_refs = self.read_refs(len)?;

                let keys_and_values_len = len
                    .checked_mul(2)
                    .ok_or_else(|| self.with_pos(ErrorKind::ObjectTooLarge))?;
                let mut child_object_refs =
                    self.allocate_vec(keys_and_values_len, self.ref_size as usize)?;
                let len = key_refs.len();
                for i in 1..=len {
                    // Reverse so we can pop off the end of the stack in order
                    child_object_refs.push(value_refs[len - i]);
                    child_object_refs.push(key_refs[len - i]);
                }

                self.push_stack_item_and_check_for_recursion(StackItem {
                    object_ref,
                    ty: StackType::Dict,
                    child_object_refs,
                })?;

                Some(Event::StartDictionary(Some(len as u64)))
            }
            (_, _) => return Err(self.with_pos(ErrorKind::UnknownObjectType(token))),
        };

        Ok(result)
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = [0; 1];
        self.reader.read_all(&mut buf)?;
        Ok(buf[0])
    }

    fn read_be_u16(&mut self) -> Result<u16, Error> {
        let mut buf = [0; 2];
        self.reader.read_all(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_be_u24(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        self.reader.read_all(&mut buf[1..])?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_be_u32(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        self.reader.read_all(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_be_u64(&mut self) -> Result<u64, Error> {
        let mut buf = [0; 8];
        self.reader.read_all(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_be_i64(&mut self) -> Result<i64, Error> {
        let mut buf = [0; 8];
        self.reader.read_all(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_be_i128(&mut self) -> Result<i128, Error> {
        let mut buf = [0; 16];
        self.reader.read_all(&mut buf)?;
        Ok(i128::from_be_bytes(buf))
    }

    fn with_pos(&self, kind: ErrorKind) -> Error {
        kind.with_byte_offset(self.reader.pos)
    }
}

impl<R: Read + Seek> Iterator for BinaryReader<R> {
    type Item = Result<OwnedEvent, Error>;

    fn next(&mut self) -> Option<Result<OwnedEvent, Error>> {
        match self.read_next() {
            Ok(Some(event)) => Some(Ok(event)),
            Err(err) => {
                // Mark the plist as finished
                self.stack.clear();
                Some(Err(err))
            }
            Ok(None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn streaming_parser() {
        use crate::stream::Event::*;

        let reader = File::open("./tests/data/binary.plist").unwrap();
        let streaming_parser = BinaryReader::new(reader);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        let comparison = &[
            StartDictionary(Some(13)),
            String("Author".into()),
            String("William Shakespeare".into()),
            String("Birthdate".into()),
            Date(super::Date::from_xml_format("1981-05-16T11:32:06Z").unwrap()),
            String("EmptyArray".into()),
            StartArray(Some(0)),
            EndCollection,
            String("IsNotFalse".into()),
            Boolean(false),
            String("SmallestNumber".into()),
            Integer((-9223372036854775808i64).into()),
            String("EmptyDictionary".into()),
            StartDictionary(Some(0)),
            EndCollection,
            String("Height".into()),
            Real(1.6),
            String("Lines".into()),
            StartArray(Some(2)),
            String("It is a tale told by an idiot,     ".into()),
            String("Full of sound and fury, signifying nothing.".into()),
            EndCollection,
            String("Death".into()),
            Integer(1564.into()),
            String("Blank".into()),
            String("".into()),
            String("BiggestNumber".into()),
            Integer(18446744073709551615u64.into()),
            String("IsTrue".into()),
            Boolean(true),
            String("Data".into()),
            Data(vec![0, 0, 0, 190, 0, 0, 0, 3, 0, 0, 0, 30, 0, 0, 0].into()),
            EndCollection,
        ];

        assert_eq!(events, &comparison[..]);
    }

    #[test]
    fn utf16_plist() {
        let reader = File::open("./tests/data/utf16_bplist.plist").unwrap();
        let streaming_parser = BinaryReader::new(reader);
        let mut events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        assert_eq!(events[2], Event::String("\u{2605} or better".into()));

        let poem = if let Event::String(ref mut poem) = events[4] {
            poem
        } else {
            panic!("not a string")
        };
        assert_eq!(poem.len(), 643);
        assert_eq!(poem.to_mut().pop().unwrap(), '\u{2605}');
    }

    #[test]
    fn nskeyedarchiver_plist() {
        let reader = File::open("./tests/data/binary_NSKeyedArchiver.plist").unwrap();
        let streaming_parser = BinaryReader::new(reader);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        assert_eq!(events[10], Event::Uid(Uid::new(4)));
        assert_eq!(events[12], Event::Uid(Uid::new(2)));
        assert_eq!(events[18], Event::Uid(Uid::new(3)));
        assert_eq!(events[46], Event::Uid(Uid::new(1)));
    }

    #[test]
    fn three_byte_integer_object_offset_plist() {
        let reader =
            File::open("./tests/data/binary_three_byte_integer_offset_table.plist").unwrap();
        let streaming_parser = BinaryReader::new(reader);
        let events: Vec<Event> = streaming_parser.map(|e| e.unwrap()).collect();

        assert_eq!(events[0], Event::StartDictionary(Some(4)));
        assert_eq!(events[1], Event::String("data".into()));
        assert_eq!(events[2], Event::StartDictionary(Some(2199)));
        assert_eq!(events[3], Event::String("1838".into()));
    }
}
