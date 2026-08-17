use std::fmt;
use std::str;

pub use self::Encoding::{Chunked, Brotli, Gzip, Deflate, Compress, Identity, EncodingExt, Trailers};

/// A value to represent an encoding used in `Transfer-Encoding`
/// or `Accept-Encoding` header.
#[derive(Clone, PartialEq, Debug)]
pub enum Encoding {
    /// The `chunked` encoding.
    Chunked,
    /// The `br` encoding.
    Brotli,
    /// The `gzip` encoding.
    Gzip,
    /// The `deflate` encoding.
    Deflate,
    /// The `compress` encoding.
    Compress,
    /// The `identity` encoding.
    Identity,
    /// The `trailers` encoding.
    Trailers,
    /// Some other encoding that is less common, can be any String.
    EncodingExt(String)
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Chunked => "chunked",
            Brotli => "br",
            Gzip => "gzip",
            Deflate => "deflate",
            Compress => "compress",
            Identity => "identity",
            Trailers => "trailers",
            EncodingExt(ref s) => s.as_ref()
        })
    }
}

impl str::FromStr for Encoding {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<Encoding> {
        match s {
            "chunked" => Ok(Chunked),
            "br" => Ok(Brotli),
            "deflate" => Ok(Deflate),
            "gzip" => Ok(Gzip),
            "compress" => Ok(Compress),
            "identity" => Ok(Identity),
            "trailers" => Ok(Trailers),
            _ => Ok(EncodingExt(s.to_owned()))
        }
    }
}
