pub trait BincodeByteOrder {
    type Endian: ::byteorder::ByteOrder + 'static;
}

/// Little-endian byte ordering.
#[derive(Copy, Clone)]
pub struct LittleEndian;

/// Big-endian byte ordering.
#[derive(Copy, Clone)]
pub struct BigEndian;

/// The native byte ordering of the current system.
#[derive(Copy, Clone)]
pub struct NativeEndian;

impl BincodeByteOrder for LittleEndian {
    type Endian = ::byteorder::LittleEndian;
}

impl BincodeByteOrder for BigEndian {
    type Endian = ::byteorder::BigEndian;
}

impl BincodeByteOrder for NativeEndian {
    type Endian = ::byteorder::NativeEndian;
}
