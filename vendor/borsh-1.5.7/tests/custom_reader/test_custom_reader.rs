use borsh::{from_reader, to_vec, BorshDeserialize, BorshSerialize};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

const ERROR_NOT_ALL_BYTES_READ: &str = "Not all bytes read";
const ERROR_UNEXPECTED_LENGTH_OF_INPUT: &str = "Unexpected length of input";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Serializable {
    item1: i32,
    item2: String,
    item3: f64,
}

#[test]
fn test_custom_reader() {
    let s = Serializable {
        item1: 100,
        item2: "foo".into(),
        item3: 1.2345,
    };
    let bytes = to_vec(&s).unwrap();
    let mut reader = CustomReader {
        data: bytes,
        read_index: 0,
    };
    let de: Serializable = BorshDeserialize::deserialize_reader(&mut reader).unwrap();
    assert_eq!(de.item1, s.item1);
    assert_eq!(de.item2, s.item2);
    assert_eq!(de.item3, s.item3);
}

#[test]
fn test_custom_reader_with_insufficient_data() {
    let s = Serializable {
        item1: 100,
        item2: "foo".into(),
        item3: 1.2345,
    };
    let mut bytes = to_vec(&s).unwrap();
    bytes.pop().unwrap();
    let mut reader = CustomReader {
        data: bytes,
        read_index: 0,
    };
    assert_eq!(
        <Serializable as BorshDeserialize>::deserialize_reader(&mut reader)
            .unwrap_err()
            .to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[test]
fn test_custom_reader_with_too_much_data() {
    let s = Serializable {
        item1: 100,
        item2: "foo".into(),
        item3: 1.2345,
    };
    let mut bytes = to_vec(&s).unwrap();
    bytes.push(1);
    let mut reader = CustomReader {
        data: bytes,
        read_index: 0,
    };
    assert_eq!(
        from_reader::<CustomReader, Serializable>(&mut reader)
            .unwrap_err()
            .to_string(),
        ERROR_NOT_ALL_BYTES_READ
    );
}

struct CustomReader {
    data: Vec<u8>,
    read_index: usize,
}

impl borsh::io::Read for CustomReader {
    fn read(&mut self, buf: &mut [u8]) -> borsh::io::Result<usize> {
        let len = buf.len().min(self.data.len() - self.read_index);
        buf[0..len].copy_from_slice(&self.data[self.read_index..self.read_index + len]);
        self.read_index += len;
        Ok(len)
    }
}

#[test]
fn test_custom_reader_that_doesnt_fill_slices() {
    let s = Serializable {
        item1: 100,
        item2: "foo".into(),
        item3: 1.2345,
    };
    let bytes = to_vec(&s).unwrap();
    let mut reader = CustomReaderThatDoesntFillSlices {
        data: bytes,
        read_index: 0,
    };
    let de: Serializable = BorshDeserialize::deserialize_reader(&mut reader).unwrap();
    assert_eq!(de.item1, s.item1);
    assert_eq!(de.item2, s.item2);
    assert_eq!(de.item3, s.item3);
}

struct CustomReaderThatDoesntFillSlices {
    data: Vec<u8>,
    read_index: usize,
}

impl borsh::io::Read for CustomReaderThatDoesntFillSlices {
    fn read(&mut self, buf: &mut [u8]) -> borsh::io::Result<usize> {
        let len = buf.len().min(self.data.len() - self.read_index);
        let len = if len <= 1 { len } else { len / 2 };
        buf[0..len].copy_from_slice(&self.data[self.read_index..self.read_index + len]);
        self.read_index += len;
        Ok(len)
    }
}

#[test]
fn test_custom_reader_that_fails_preserves_error_information() {
    let mut reader = CustomReaderThatFails;

    let err = from_reader::<CustomReaderThatFails, Serializable>(&mut reader).unwrap_err();
    assert_eq!(err.to_string(), "I don't like to run");
    assert_eq!(err.kind(), borsh::io::ErrorKind::ConnectionAborted);
}

struct CustomReaderThatFails;

impl borsh::io::Read for CustomReaderThatFails {
    fn read(&mut self, _buf: &mut [u8]) -> borsh::io::Result<usize> {
        Err(borsh::io::Error::new(
            borsh::io::ErrorKind::ConnectionAborted,
            "I don't like to run",
        ))
    }
}
