mod array;
mod bin;
mod bool;
mod ext;
mod float;
mod map;
mod null;
mod sint;
mod string;
mod uint;

#[cfg(feature = "std")]
pub type Cursor<'a> = std::io::Cursor<&'a [u8]>;
#[cfg(not(feature = "std"))]
pub type Cursor<'a> = rmp::decode::Bytes<'a>;
