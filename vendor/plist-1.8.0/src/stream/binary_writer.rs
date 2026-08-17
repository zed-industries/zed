// TODO: Revisit the design of `Event` once the `HashMap` raw interface is stabilised.
// Ideally `Value`s would be stored inline in `Event`.

use indexmap::IndexMap;
use std::{
    borrow::Cow,
    io::{self, Write},
    num::NonZeroUsize,
};

use crate::{
    error::{self, Error, ErrorKind, EventKind},
    stream::Writer,
    Date, Integer, Uid,
};

pub struct BinaryWriter<W: Write> {
    writer: PosWriter<W>,
    events: Vec<Event>,
    dictionary_key_events: Vec<usize>,
    values: IndexMap<Value<'static>, ValueState>,
    /// Pointers into `events` for each of the currently unclosed `Collection` events.
    collection_stack: Vec<usize>,
    /// The number of `Collection` and unique `Value` events in `events`.
    num_objects: usize,
}

struct PosWriter<W: Write> {
    writer: W,
    pos: usize,
}

#[derive(Clone)]
struct ObjectRef(NonZeroUsize);

/// An array of `len` elements is stored as a `Collection` event followed by `skip_len` events
/// containing the contents of the array. e.g.
///
/// Collection(ty: Array, len: 2, skip_len: 2)
/// Value
/// Value
///
/// If the array contains another array or dictionary `len` and `skip_len` will differ. e.g.
///
/// Collection(ty: Array, len: 2, skip_len: 3)
/// Value
/// Collection(ty: Array, len: 1, skip_len: 1)
/// Value
///
/// A dictionary of `len` (key, value) pairs is stored as a `Collection` event followed by
/// `skip_len` events containing the contents of the dictionary. The dictionary values are stored
/// first. These are followed by a `DictionaryKeys` event and then the keys themselves. e.g.
///
/// Collection(ty: Dictionary, len: 2, skip_len: 6)
/// Value
/// Collection(ty: Array, len: 1, skip_len: 1)
/// Value
/// DictionaryKeys(2)
/// Value (Key)
/// Value (Key)
///
/// This arrangement simplifies writing dictionaries as they must be written in the order
/// (key, key, value, value) instead of (key, value, key, value) as they are passed to the writer.
/// Unclosed dictionaries have their keys stored in `dictionary_key_events` and these are only
/// moved to the end of the `BinaryWriter::events` array once the dictionary is closed in
/// `write_end_collection`.
enum Event {
    Collection(Collection),
    /// Index of the value in the `values` map.
    Value(usize),
    /// The number of dictionary keys following this event.
    DictionaryKeys(usize),
}

struct Collection {
    ty: CollectionType,
    /// The number of elements in an array or (key, value) pairs in a dictionary.
    /// Unclosed dictionaries have a `len` equal to the number of keys plus the number of values
    /// written so far. This is fixed up in `write_end_collection`.
    len: usize,
    /// The number of events to skip to get to the next element after the collection.
    skip: usize,
    object_ref: Option<ObjectRef>,
}

#[derive(Eq, PartialEq)]
enum CollectionType {
    Array,
    Dictionary,
}

#[derive(Eq, Hash, PartialEq)]
enum Value<'a> {
    Boolean(bool),
    Data(Cow<'a, [u8]>),
    Date(Date),
    Integer(Integer),
    /// Floats are deduplicated based on their bitwise value.
    Real(u64),
    String(Cow<'a, str>),
    Uid(Uid),
}

enum ValueState {
    /// The value has not been assigned an object reference.
    Unassigned,
    /// The value has been assigned an object reference but has not yet been written.
    Unwritten(ObjectRef),
    /// The value has been written with the given object reference.
    Written(ObjectRef),
}

impl<W: Write> BinaryWriter<W> {
    pub fn new(writer: W) -> BinaryWriter<W> {
        BinaryWriter {
            writer: PosWriter { writer, pos: 0 },
            events: Vec::new(),
            dictionary_key_events: Vec::new(),
            values: IndexMap::new(),
            collection_stack: Vec::new(),
            num_objects: 0,
        }
    }

    fn write_start_collection(&mut self, ty: CollectionType) -> Result<(), Error> {
        if self.expecting_dictionary_key() {
            let ty_event_kind = match ty {
                CollectionType::Array => EventKind::StartArray,
                CollectionType::Dictionary => EventKind::StartDictionary,
            };
            return Err(ErrorKind::UnexpectedEventType {
                expected: EventKind::DictionaryKeyOrEndCollection,
                found: ty_event_kind,
            }
            .without_position());
        }
        self.increment_current_collection_len();
        self.collection_stack.push(self.events.len());
        self.events.push(Event::Collection(Collection {
            ty,
            len: 0,
            skip: 0,
            object_ref: None,
        }));
        self.num_objects += 1;
        Ok(())
    }

    fn write_end_collection(&mut self) -> Result<(), Error> {
        let collection_event_index = self.collection_stack.pop().ok_or_else(|| {
            ErrorKind::UnexpectedEventType {
                expected: EventKind::ValueOrStartCollection,
                found: EventKind::EndCollection,
            }
            .without_position()
        })?;

        let current_event_index = self.events.len() - 1;
        let Event::Collection(c) = &mut self.events[collection_event_index] else {
            unreachable!("items in `collection_stack` always point to a collection event");
        };

        c.skip = current_event_index - collection_event_index;

        if let CollectionType::Dictionary = c.ty {
            // Ensure that every dictionary key is paired with a value.
            if !is_even(c.len) {
                return Err(ErrorKind::UnexpectedEventType {
                    expected: EventKind::DictionaryKeyOrEndCollection,
                    found: EventKind::EndCollection,
                }
                .without_position());
            }

            // Fix up the dictionary length. It should contain the number of key-value pairs,
            // not the number of keys and values.
            c.len /= 2;

            // To skip past a dictionary we also need to skip the `DictionaryKeys` event and the
            // keys that follow it.
            c.skip += 1 + c.len;
            let len = c.len;
            self.events.push(Event::DictionaryKeys(len));

            // Move the cached dictionary keys to the end of the events array.
            let keys_start_index = self.dictionary_key_events.len() - len;
            self.events.extend(
                self.dictionary_key_events
                    .drain(keys_start_index..)
                    .map(Event::Value),
            );
        }

        if self.collection_stack.is_empty() {
            self.write_plist()?;
        }

        Ok(())
    }

    fn write_value(&mut self, value: Value) -> Result<(), Error> {
        let expecting_dictionary_key = self.expecting_dictionary_key();

        // Ensure that all dictionary keys are strings.
        match (&value, expecting_dictionary_key) {
            (Value::String(_), true) | (_, false) => (),
            (_, true) => {
                return Err(ErrorKind::UnexpectedEventType {
                    expected: EventKind::DictionaryKeyOrEndCollection,
                    found: value.event_kind(),
                }
                .without_position())
            }
        }

        // Deduplicate `value`. There is one entry in `values` for each unqiue `Value` in the
        // plist.
        let value_index = if let Some((value_index, _, _)) = self.values.get_full(&value) {
            value_index
        } else {
            self.num_objects += 1;
            let value = value.into_owned();
            let (value_index, _) = self.values.insert_full(value, ValueState::Unassigned);
            value_index
        };

        // Dictionary keys are buffered in `dictionary_key_events` until the dictionary is closed
        // in `write_end_collection` when they are moved to the end of the `events` array.
        if expecting_dictionary_key {
            self.dictionary_key_events.push(value_index);
        } else {
            self.events.push(Event::Value(value_index));
        }

        self.increment_current_collection_len();

        if self.collection_stack.is_empty() {
            self.write_plist()?;
        }

        Ok(())
    }

    fn expecting_dictionary_key(&self) -> bool {
        if let Some(&event_index) = self.collection_stack.last() {
            if let Event::Collection(c) = &self.events[event_index] {
                c.ty == CollectionType::Dictionary && is_even(c.len)
            } else {
                unreachable!("items in `collection_stack` always point to a collection event");
            }
        } else {
            false
        }
    }

    fn increment_current_collection_len(&mut self) {
        if let Some(&event_index) = self.collection_stack.last() {
            if let Event::Collection(c) = &mut self.events[event_index] {
                c.len += 1;
            } else {
                unreachable!("items in `collection_stack` always point to a collection event");
            }
        }
    }

    fn write_plist(&mut self) -> Result<(), Error> {
        assert!(self.collection_stack.is_empty());

        // Write header
        self.writer.write_exact(b"bplist00")?;

        // Write objects
        let mut events_vec = std::mem::take(&mut self.events);
        let mut events = &mut events_vec[..];
        let ref_size = plist_ref_size(self.num_objects - 1);
        let mut offset_table = vec![0; self.num_objects];

        // Assign the first (root) event an object reference of zero.
        let mut next_object_ref = ObjectRef::zero();
        match &mut events[0] {
            Event::Value(value_index) => {
                let (_, value_state) = value_mut(&mut self.values, *value_index);
                *value_state = ValueState::Unwritten(next_object_ref.clone_and_increment_self());
            }
            Event::Collection(c) => {
                c.object_ref = Some(next_object_ref.clone_and_increment_self());
            }
            Event::DictionaryKeys(_) => {
                unreachable!("`events` starts with a value or collection event")
            }
        }

        while let Some((event, rest)) = events.split_first_mut() {
            events = rest;
            match event {
                Event::Collection(c) => {
                    let collection_events = &mut events[..c.skip];
                    self.write_plist_collection(
                        c,
                        collection_events,
                        ref_size,
                        &mut next_object_ref,
                        &mut offset_table,
                    )?;
                }
                Event::Value(value_index) => {
                    self.write_plist_value(*value_index, &mut offset_table)?;
                }
                // Dictionary keys will have already been written in `write_plist_collection` so we
                // skip over them here.
                Event::DictionaryKeys(len) => {
                    events = &mut events[*len..];
                }
            }
        }

        // Write object offset table
        let offset_table_offset = self.writer.pos;
        let offset_size = plist_ref_size(offset_table_offset);
        for &offset in &offset_table {
            write_plist_ref(&mut self.writer, offset_size, offset)?;
        }

        // Write trailer
        // 6 zero bytes padding
        // 1 byte offset size
        // 1 byte object ref size
        // 8 bytes number of objects
        // 8 bytes root object ref (always zero)
        // 8 bytes file offset of the object offset table
        let mut trailer = [0; 32];
        trailer[6] = offset_size;
        trailer[7] = ref_size;
        trailer[8..16].copy_from_slice(&(self.num_objects as u64).to_be_bytes());
        trailer[24..32].copy_from_slice(&(offset_table_offset as u64).to_be_bytes());
        self.writer.write_exact(&trailer)?;

        self.writer
            .flush()
            .map_err(error::from_io_without_position)?;

        // Reset plist writer
        self.writer.pos = 0;
        events_vec.clear();
        self.events = events_vec;
        self.values.clear();
        self.num_objects = 0;

        Ok(())
    }

    fn write_plist_collection(
        &mut self,
        collection: &Collection,
        events: &mut [Event],
        ref_size: u8,
        next_object_ref: &mut ObjectRef,
        offset_table: &mut [usize],
    ) -> Result<(), Error> {
        if let Some(object_ref) = &collection.object_ref {
            offset_table[object_ref.value()] = self.writer.pos;
        } else {
            unreachable!("collection object refs are assigned before this function is called");
        }

        // Split the events in the current collection into keys and values (arrays contain only
        // values). This is required as dictionary keys appear after values in the `events array
        // but all keys must be written before any values.
        let (keys, values, ty) = match collection.ty {
            CollectionType::Array => (&mut [][..], events, 0xa0),
            CollectionType::Dictionary => {
                let keys_start_offset = events.len() - collection.len - 1;
                let (values, keys) = events.split_at_mut(keys_start_offset);
                (&mut keys[1..], values, 0xd0)
            }
        };
        let mut collection_events = keys.iter_mut().chain(values);

        // Collections are written as a length prefixed array of object references. For an array
        // the length is the number of elements. For a dictionary it is the number of (key, value)
        // pairs.
        write_plist_value_ty_and_size(&mut self.writer, ty, collection.len)?;
        while let Some(event) = collection_events.next() {
            let object_ref = match event {
                Event::Collection(c) => {
                    // We only want to write references to top level elements in the collection so
                    // we skip over the contents of any sub-collections.
                    if c.skip > 0 {
                        let _ = collection_events.nth(c.skip - 1);
                    }

                    // Collections are not deduplicated so they must be assigned an object
                    // reference here.
                    assert!(c.object_ref.is_none());
                    let object_ref = next_object_ref.clone_and_increment_self();
                    c.object_ref = Some(object_ref.clone());
                    object_ref
                }
                Event::Value(value_index) => {
                    // Values are deduplicated so we only assign an object reference if we have not
                    // already done so previously.
                    let (_, value_state) = value_mut(&mut self.values, *value_index);
                    match value_state {
                        ValueState::Unassigned => {
                            let object_ref = next_object_ref.clone_and_increment_self();
                            *value_state = ValueState::Unwritten(object_ref.clone());
                            object_ref
                        }
                        ValueState::Unwritten(object_ref) | ValueState::Written(object_ref) => {
                            object_ref.clone()
                        }
                    }
                }
                Event::DictionaryKeys(_) => unreachable!(
                    "`DictionaryKeys` events are specifically excluded from the iterator"
                ),
            };
            write_plist_ref(&mut self.writer, ref_size, object_ref.value())?;
        }

        // We write dictionary keys here as they appear after values in the `events` array but
        // should come before values in the plist stream to reduce seeking on read.
        for key in keys {
            if let Event::Value(value_index) = key {
                self.write_plist_value(*value_index, offset_table)?;
            } else {
                unreachable!("dictionary keys are assigned as values in `write_end_collection`");
            }
        }

        Ok(())
    }

    fn write_plist_value(
        &mut self,
        value_index: usize,
        offset_table: &mut [usize],
    ) -> Result<(), Error> {
        let (value, value_state) = value_mut(&mut self.values, value_index);

        let object_ref = match value_state {
            ValueState::Unassigned => {
                unreachable!("value object refs are assigned before this function is called");
            }
            ValueState::Unwritten(object_ref) => object_ref.clone(),
            ValueState::Written(_) => return Ok(()),
        };

        offset_table[object_ref.value()] = self.writer.pos;
        *value_state = ValueState::Written(object_ref);

        match value {
            Value::Boolean(true) => {
                self.writer.write_exact(&[0x09])?;
            }
            Value::Boolean(false) => {
                self.writer.write_exact(&[0x08])?;
            }
            Value::Data(v) => {
                write_plist_value_ty_and_size(&mut self.writer, 0x40, v.len())?;
                self.writer.write_exact(&v[..])?;
            }
            Value::Date(v) => {
                let secs = v.as_seconds_since_plist_epoch();
                let mut buf: [_; 9] = [0x33, 0, 0, 0, 0, 0, 0, 0, 0];
                buf[1..].copy_from_slice(&secs.to_bits().to_be_bytes());
                self.writer.write_exact(&buf)?;
            }
            Value::Integer(v) => {
                if let Some(v) = v.as_signed() {
                    if let Ok(v) = u8::try_from(v) {
                        self.writer.write_exact(&[0x10, v])?;
                    } else if let Ok(v) = u16::try_from(v) {
                        let mut buf: [_; 3] = [0x11, 0, 0];
                        buf[1..].copy_from_slice(&v.to_be_bytes());
                        self.writer.write_exact(&buf)?;
                    } else if let Ok(v) = u32::try_from(v) {
                        let mut buf: [_; 5] = [0x12, 0, 0, 0, 0];
                        buf[1..].copy_from_slice(&v.to_be_bytes());
                        self.writer.write_exact(&buf)?;
                    } else {
                        let mut buf: [_; 9] = [0x13, 0, 0, 0, 0, 0, 0, 0, 0];
                        buf[1..].copy_from_slice(&v.to_be_bytes());
                        self.writer.write_exact(&buf)?;
                    }
                } else if let Some(v) = v.as_unsigned() {
                    // `u64`s larger than `i64::MAX` are stored as signed 128 bit
                    // integers.
                    let mut buf: [_; 17] = [0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    buf[1..].copy_from_slice(&i128::from(v).to_be_bytes());
                    self.writer.write_exact(&buf)?;
                } else {
                    unreachable!("an integer can be represented as either an i64 or u64");
                }
            }
            Value::Real(v) => {
                let mut buf: [_; 9] = [0x23, 0, 0, 0, 0, 0, 0, 0, 0];
                buf[1..].copy_from_slice(&v.to_be_bytes());
                self.writer.write_exact(&buf)?;
            }
            Value::String(v) if v.is_ascii() => {
                let ascii = v.as_bytes();
                write_plist_value_ty_and_size(&mut self.writer, 0x50, ascii.len())?;
                self.writer.write_exact(ascii)?;
            }
            Value::String(v) => {
                let utf16_len = v.encode_utf16().count();
                write_plist_value_ty_and_size(&mut self.writer, 0x60, utf16_len)?;
                for c in v.encode_utf16() {
                    self.writer.write_exact(&c.to_be_bytes())?;
                }
            }
            Value::Uid(v) => {
                let v = v.get();
                if let Ok(v) = u8::try_from(v) {
                    self.writer.write_exact(&[0x80, v])?;
                } else if let Ok(v) = u16::try_from(v) {
                    let mut buf: [_; 3] = [0x81, 0, 0];
                    buf[1..].copy_from_slice(&v.to_be_bytes());
                    self.writer.write_exact(&buf)?;
                } else if let Ok(v) = u32::try_from(v) {
                    let mut buf: [_; 5] = [0x83, 0, 0, 0, 0];
                    buf[1..].copy_from_slice(&v.to_be_bytes());
                    self.writer.write_exact(&buf)?;
                } else {
                    let mut buf: [_; 9] = [0x87, 0, 0, 0, 0, 0, 0, 0, 0];
                    // we want to be explicit about the type here
                    #[allow(clippy::unnecessary_cast)]
                    buf[1..].copy_from_slice(&(v as u64).to_be_bytes());
                    self.writer.write_exact(&buf)?;
                }
            }
        }
        Ok(())
    }
}

impl<W: Write> Writer for BinaryWriter<W> {
    fn write_start_array(&mut self, _len: Option<u64>) -> Result<(), Error> {
        self.write_start_collection(CollectionType::Array)
    }
    fn write_start_dictionary(&mut self, _len: Option<u64>) -> Result<(), Error> {
        self.write_start_collection(CollectionType::Dictionary)
    }
    fn write_end_collection(&mut self) -> Result<(), Error> {
        self.write_end_collection()
    }

    fn write_boolean(&mut self, value: bool) -> Result<(), Error> {
        self.write_value(Value::Boolean(value))
    }
    fn write_data(&mut self, value: Cow<[u8]>) -> Result<(), Error> {
        self.write_value(Value::Data(value))
    }
    fn write_date(&mut self, value: Date) -> Result<(), Error> {
        self.write_value(Value::Date(value))
    }
    fn write_integer(&mut self, value: Integer) -> Result<(), Error> {
        self.write_value(Value::Integer(value))
    }
    fn write_real(&mut self, value: f64) -> Result<(), Error> {
        self.write_value(Value::Real(value.to_bits()))
    }
    fn write_string(&mut self, value: Cow<str>) -> Result<(), Error> {
        self.write_value(Value::String(value))
    }
    fn write_uid(&mut self, value: Uid) -> Result<(), Error> {
        self.write_value(Value::Uid(value))
    }
}

fn is_even(value: usize) -> bool {
    value & 1 == 0
}

fn value_mut<'a>(
    values: &'a mut IndexMap<Value<'static>, ValueState>,
    value_index: usize,
) -> (&'a Value<'static>, &'a mut ValueState) {
    values
        .get_index_mut(value_index)
        .expect("internal consistency error")
}

fn write_plist_value_ty_and_size(
    writer: &mut PosWriter<impl Write>,
    token: u8,
    size: usize,
) -> Result<(), Error> {
    if let Ok(size) = u8::try_from(size) {
        if size < 0x0f {
            writer.write_exact(&[token | size])?;
        } else {
            writer.write_exact(&[token | 0x0f, 0x10, size])?;
        }
    } else if let Ok(size) = u16::try_from(size) {
        let mut buf: [_; 4] = [token | 0x0f, 0x11, 0, 0];
        buf[2..].copy_from_slice(&size.to_be_bytes());
        writer.write_exact(&buf)?;
    } else if let Ok(size) = u32::try_from(size) {
        let mut buf: [_; 6] = [token | 0x0f, 0x12, 0, 0, 0, 0];
        buf[2..].copy_from_slice(&size.to_be_bytes());
        writer.write_exact(&buf)?;
    } else {
        let mut buf: [_; 10] = [token | 0x0f, 0x13, 0, 0, 0, 0, 0, 0, 0, 0];
        buf[2..].copy_from_slice(&(size as u64).to_be_bytes());
        writer.write_exact(&buf)?;
    }
    Ok(())
}

fn plist_ref_size(max_value: usize) -> u8 {
    let significant_bits = 64 - (max_value as u64).leading_zeros() as u8;
    // Convert to number of bytes
    let significant_bytes = (significant_bits + 7) / 8;
    // Round up to the next integer byte size which must be power of two.
    significant_bytes.next_power_of_two()
}

fn write_plist_ref(
    writer: &mut PosWriter<impl Write>,
    ref_size: u8,
    value: usize,
) -> Result<(), Error> {
    match ref_size {
        1 => writer.write_exact(&[value as u8]),
        2 => writer.write_exact(&(value as u16).to_be_bytes()),
        4 => writer.write_exact(&(value as u32).to_be_bytes()),
        8 => writer.write_exact(&(value as u64).to_be_bytes()),
        _ => unreachable!("`ref_size` is a power of two less than or equal to 8"),
    }
}

impl<W: Write> PosWriter<W> {
    fn write_exact(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.write_all(buf)
            .map_err(error::from_io_without_position)?;
        Ok(())
    }
}

impl<W: Write> Write for PosWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let count = self.writer.write(buf)?;
        self.pos = self
            .pos
            .checked_add(count)
            .expect("binary plist cannot be larger than `usize::MAX` bytes");
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl ObjectRef {
    fn zero() -> ObjectRef {
        ObjectRef(NonZeroUsize::new(1).unwrap())
    }

    fn clone_and_increment_self(&mut self) -> ObjectRef {
        let current = self.0;
        self.0 = NonZeroUsize::new(current.get() + 1).unwrap();
        ObjectRef(current)
    }

    fn value(&self) -> usize {
        self.0.get() - 1
    }
}

impl Value<'_> {
    fn into_owned(self) -> Value<'static> {
        match self {
            Value::Boolean(v) => Value::Boolean(v),
            Value::Data(v) => Value::Data(Cow::Owned(v.into_owned())),
            Value::Date(v) => Value::Date(v),
            Value::Integer(v) => Value::Integer(v),
            Value::Real(v) => Value::Real(v),
            Value::String(v) => Value::String(Cow::Owned(v.into_owned())),
            Value::Uid(v) => Value::Uid(v),
        }
    }

    fn event_kind(&self) -> EventKind {
        match self {
            Value::Boolean(_) => EventKind::Boolean,
            Value::Data(_) => EventKind::Data,
            Value::Date(_) => EventKind::Date,
            Value::Integer(_) => EventKind::Integer,
            Value::Real(_) => EventKind::Real,
            Value::String(_) => EventKind::String,
            Value::Uid(_) => EventKind::Uid,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Cursor, path::Path};

    use crate::{stream::BinaryReader, Value};

    fn test_roundtrip<P: AsRef<Path>>(path: P) {
        let reader = File::open(path).unwrap();
        let streaming_parser = BinaryReader::new(reader);
        let value_to_encode = Value::from_events(streaming_parser).unwrap();

        let mut buf = Cursor::new(Vec::new());
        value_to_encode.to_writer_binary(&mut buf).unwrap();

        let buf_inner = buf.into_inner();

        let streaming_parser = BinaryReader::new(Cursor::new(buf_inner));

        let events: Vec<Result<_, _>> = streaming_parser.collect();
        let value_decoded_from_encode = Value::from_events(events).unwrap();

        assert_eq!(value_to_encode, value_decoded_from_encode);
    }

    #[test]
    fn bplist_roundtrip() {
        test_roundtrip("./tests/data/binary.plist")
    }

    #[test]
    fn utf16_roundtrip() {
        test_roundtrip("./tests/data/utf16_bplist.plist")
    }

    #[test]
    fn nskeyedarchiver_roundtrip() {
        test_roundtrip("./tests/data/binary_NSKeyedArchiver.plist")
    }
}
