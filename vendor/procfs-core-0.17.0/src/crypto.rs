use crate::{expect, FromBufRead, ProcError, ProcResult};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::BufRead,
    iter::{once, Peekable},
    str::FromStr,
};

/// Represents the data from `/proc/crypto`.
///
/// Each block represents a cryptographic implementation that has been registered with the kernel.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct CryptoTable {
    pub crypto_blocks: HashMap<String, Vec<CryptoBlock>>,
}

/// Format of a crypto implementation represented in /proc/crypto.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct CryptoBlock {
    pub name: String,
    pub driver: String,
    pub module: String,
    pub priority: isize,
    pub ref_count: isize,
    pub self_test: SelfTest,
    pub internal: bool,
    pub fips_enabled: bool,
    pub crypto_type: Type,
}

impl FromBufRead for CryptoTable {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut lines = r.lines().peekable();
        let mut crypto_blocks: HashMap<String, Vec<CryptoBlock>> = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line?;
            // Just skip empty lines
            if !line.is_empty() {
                let mut split = line.split(':');
                let name = expect!(split.next());
                if name.trim() == "name" {
                    let name = expect!(split.next()).trim().to_string();
                    let block = CryptoBlock::from_iter(&mut lines, name.as_str())?;
                    let blocks = crypto_blocks.entry(name).or_insert(Vec::new());
                    blocks.push(block);
                }
            }
        }

        Ok(CryptoTable { crypto_blocks })
    }
}

impl CryptoTable {
    pub fn get<T: AsRef<str>>(&self, target: T) -> Option<&Vec<CryptoBlock>> {
        self.crypto_blocks.get(target.as_ref())
    }
}

impl CryptoBlock {
    fn from_iter<T: Iterator<Item = Result<String, std::io::Error>>>(
        iter: &mut Peekable<T>,
        name: &str,
    ) -> ProcResult<Self> {
        let driver = parse_line(iter, "driver", name)?;
        let module = parse_line(iter, "module", name)?;
        let priority = from_str!(isize, &parse_line(iter, "priority", name)?);
        let ref_count = from_str!(isize, &parse_line(iter, "refcnt", name)?);
        let self_test = SelfTest::try_from(parse_line(iter, "selftest", name)?.as_str())?;
        let internal = parse_bool(iter, "internal", name)?;
        let fips_enabled = parse_fips(iter, name)?;
        let crypto_type = Type::from_iter(iter, name)?;
        Ok(CryptoBlock {
            name: name.to_string(),
            driver,
            module,
            priority,
            ref_count,
            self_test,
            internal,
            fips_enabled,
            crypto_type,
        })
    }
}

/// Potential results for selftest.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum SelfTest {
    Passed,
    Unknown,
}

impl TryFrom<&str> for SelfTest {
    type Error = ProcError;

    fn try_from(value: &str) -> Result<Self, ProcError> {
        Ok(match value {
            "passed" => Self::Passed,
            "unknown" => Self::Unknown,
            _ => {
                return Err(build_internal_error!(format!(
                    "Could not recognise self test string {value}"
                )))
            }
        })
    }
}

/// Enumeration of potential types and their associated data. Unknown at end to catch unrecognised types.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum Type {
    /// Symmetric Key Cipher
    Skcipher(Skcipher),
    /// Single Block Cipher
    Cipher(Cipher),
    /// Syncronous Hash
    Shash(Shash),
    /// Asyncronous Hash
    Ahash(Ahash),
    /// Authenticated Encryption with Associated Data
    Aead(Aead),
    /// Random Number Generator
    Rng(Rng),
    /// Test algorithm
    Larval(Larval),
    /// Synchronous Compression
    Scomp,
    /// General Compression
    Compression,
    /// Asymmetric Cipher
    AkCipher,
    /// Key-agreement Protocol Primitive
    Kpp,
    /// Signature
    Sig,
    /// Unrecognised type, associated data collected in to a hash map
    Unknown(Unknown),
}

impl Type {
    fn from_iter<T: Iterator<Item = Result<String, std::io::Error>>>(
        iter: &mut Peekable<T>,
        name: &str,
    ) -> ProcResult<Self> {
        let type_name = parse_line(iter, "type", name)?;
        Ok(match type_name.as_str() {
            "skcipher" => Self::Skcipher(Skcipher::parse(iter, name)?),
            "cipher" => Self::Cipher(Cipher::parse(iter, name)?),
            "shash" => Self::Shash(Shash::parse(iter, name)?),
            "scomp" => Self::Scomp,
            "compression" => Self::Compression,
            "akcipher" => Self::AkCipher,
            "kpp" => Self::Kpp,
            "ahash" => Self::Ahash(Ahash::parse(iter, name)?),
            "aead" => Self::Aead(Aead::parse(iter, name)?),
            "rng" => Self::Rng(Rng::parse(iter, name)?),
            "larval" => Self::Larval(Larval::parse(iter, name)?),
            "sig" => Self::Sig,
            unknown_name => Self::Unknown(Unknown::parse(iter, unknown_name)),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Skcipher {
    pub async_capable: bool,
    pub block_size: usize,
    pub min_key_size: usize,
    pub max_key_size: usize,
    pub iv_size: usize,
    pub chunk_size: usize,
    pub walk_size: usize,
}

impl Skcipher {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let async_capable = parse_bool(iter, "async", name)?;
        let block_size = from_str!(usize, &parse_line(iter, "blocksize", name)?);
        let min_key_size = from_str!(usize, &parse_line(iter, "min keysize", name)?);
        let max_key_size = from_str!(usize, &parse_line(iter, "max keysize", name)?);
        let iv_size = from_str!(usize, &parse_line(iter, "ivsize", name)?);
        let chunk_size = from_str!(usize, &parse_line(iter, "chunksize", name)?);
        let walk_size = from_str!(usize, &parse_line(iter, "walksize", name)?);
        Ok(Self {
            async_capable,
            block_size,
            min_key_size,
            max_key_size,
            iv_size,
            chunk_size,
            walk_size,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Cipher {
    pub block_size: usize,
    pub min_key_size: usize,
    pub max_key_size: usize,
}

impl Cipher {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let block_size = from_str!(usize, &parse_line(iter, "blocksize", name)?);
        let min_key_size = from_str!(usize, &parse_line(iter, "min keysize", name)?);
        let max_key_size = from_str!(usize, &parse_line(iter, "max keysize", name)?);
        Ok(Self {
            block_size,
            min_key_size,
            max_key_size,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Shash {
    pub block_size: usize,
    pub digest_size: usize,
}

impl Shash {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let block_size = from_str!(usize, &parse_line(iter, "blocksize", name)?);
        let digest_size = from_str!(usize, &parse_line(iter, "digestsize", name)?);
        Ok(Self {
            block_size,
            digest_size,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Ahash {
    pub async_capable: bool,
    pub block_size: usize,
    pub digest_size: usize,
}

impl Ahash {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let async_capable = parse_bool(iter, "async", name)?;
        let block_size = from_str!(usize, &parse_line(iter, "blocksize", name)?);
        let digest_size = from_str!(usize, &parse_line(iter, "digestsize", name)?);
        Ok(Self {
            async_capable,
            block_size,
            digest_size,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Aead {
    pub async_capable: bool,
    pub block_size: usize,
    pub iv_size: usize,
    pub max_auth_size: usize,
    pub gen_iv: Option<usize>,
}

impl Aead {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(
        iter: &mut Peekable<T>,
        name: &str,
    ) -> ProcResult<Self> {
        let async_capable = parse_bool(iter, "async", name)?;
        let block_size = from_str!(usize, &parse_line(iter, "blocksize", name)?);
        let iv_size = from_str!(usize, &parse_line(iter, "ivsize", name)?);
        let max_auth_size = from_str!(usize, &parse_line(iter, "maxauthsize", name)?);
        let gen_iv = parse_gen_iv(iter, name)?;
        Ok(Self {
            async_capable,
            block_size,
            iv_size,
            max_auth_size,
            gen_iv,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Rng {
    pub seed_size: usize,
}

impl Rng {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let seed_size = from_str!(usize, &parse_line(iter, "seedsize", name)?);
        Ok(Self { seed_size })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Larval {
    pub flags: u32,
}

impl Larval {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, name: &str) -> ProcResult<Self> {
        let flags = from_str!(u32, &parse_line(iter, "flags", name)?);
        Ok(Self { flags })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Unknown {
    pub fields: HashMap<String, String>,
}

impl Unknown {
    fn parse<T: Iterator<Item = Result<String, std::io::Error>>>(iter: &mut T, unknown_name: &str) -> Self {
        let fields = iter
            .map_while(|line| {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => return None,
                };
                (!line.is_empty()).then(|| {
                    line.split_once(':')
                        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                })
            })
            .flatten()
            .chain(once((String::from("name"), unknown_name.to_string())))
            .collect();
        Self { fields }
    }
}

fn parse_line<T: Iterator<Item = Result<String, std::io::Error>>>(
    iter: &mut T,
    to_find: &str,
    name: &str,
) -> ProcResult<String> {
    let line = expect!(iter.next())?;
    let (key, val) = expect!(line.split_once(':'));
    if key.trim() != to_find {
        return Err(build_internal_error!(format!(
            "could not locate {to_find} in /proc/crypto, block {name}"
        )));
    }
    Ok(val.trim().to_string())
}

fn parse_fips<T: Iterator<Item = Result<String, std::io::Error>>>(
    iter: &mut Peekable<T>,
    name: &str,
) -> ProcResult<bool> {
    if iter
        .peek()
        .map(|line| line.as_ref().is_ok_and(|line| line.contains("fips")))
        .unwrap_or(false)
    {
        let fips = parse_line(iter, "fips", name)?;
        if fips == "yes" {
            return Ok(true);
        }
    }
    Ok(false)
}

fn parse_bool<T: Iterator<Item = Result<String, std::io::Error>>>(
    iter: &mut T,
    to_find: &str,
    name: &str,
) -> ProcResult<bool> {
    match parse_line(iter, to_find, name)?.as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(build_internal_error!(format!(
            "{to_find} for {name} was unrecognised term"
        ))),
    }
}

fn parse_gen_iv<T: Iterator<Item = Result<String, std::io::Error>>>(
    iter: &mut Peekable<T>,
    name: &str,
) -> ProcResult<Option<usize>> {
    if iter
        .peek()
        .map(|line| line.as_ref().is_ok_and(|line| line.contains("geniv")))
        .unwrap_or(false)
    {
        let val = parse_line(iter, "geniv", name)?;
        if val != "<none>" {
            return Ok(Some(expect!(usize::from_str(&val))));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_correct() {
        let line = Ok("name         : ghash".to_string());
        let mut iter = std::iter::once(line);
        let val = match parse_line(&mut iter, "name", "parse_line_correct") {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
        assert_eq!("ghash", val);
    }

    #[test]
    fn parse_line_incorrect() {
        let line = Ok("name         : ghash".to_string());
        let mut iter = std::iter::once(line);
        let val = match parse_line(&mut iter, "name", "parse_line_incorrect") {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
        assert_ne!("hash", val);
    }

    #[test]
    fn parse_block() {
        let block = r#"driver       : deflate-generic
module       : kernel
priority     : 0
refcnt       : 2
selftest     : passed
internal     : no
type         : compression"#;
        let mut iter = block.lines().map(|s| Ok(s.to_string())).peekable();
        let block = CryptoBlock::from_iter(&mut iter, "deflate");
        let block = block.expect("Should be have read one block");
        assert_eq!(block.name, "deflate");
        assert_eq!(block.driver, "deflate-generic");
        assert_eq!(block.module, "kernel");
        assert_eq!(block.priority, 0);
        assert_eq!(block.ref_count, 2);
        assert_eq!(block.self_test, SelfTest::Passed);
        assert_eq!(block.internal, false);
        assert_eq!(block.crypto_type, Type::Compression);
    }

    #[test]
    fn parse_bad_block() {
        let block = r#"driver       : deflate-generic
module       : kernel
priority     : 0
refcnt       : 2
selftest     : passed
internal     : no
type         : aead"#;
        let mut iter = block.lines().map(|s| Ok(s.to_string())).peekable();
        let block = CryptoBlock::from_iter(&mut iter, "deflate");
        eprintln!("{block:?}");
        assert!(block.is_err());
    }

    #[test]
    fn parse_two() {
        let block = r#"name         : ccm(aes)
driver       : ccm_base(ctr(aes-aesni),cbcmac(aes-aesni))
module       : ccm
priority     : 300
refcnt       : 4
selftest     : passed
internal     : no
type         : aead
async        : no
blocksize    : 1
ivsize       : 16
maxauthsize  : 16
geniv        : <none>

name         : ctr(aes)
driver       : ctr(aes-aesni)
module       : kernel
priority     : 300
refcnt       : 4
selftest     : passed
internal     : no
type         : skcipher
async        : no
blocksize    : 1
min keysize  : 16
max keysize  : 32
ivsize       : 16
chunksize    : 16
walksize     : 16

"#;
        let blocks = CryptoTable::from_buf_read(block.as_bytes());
        let blocks = blocks.expect("Should be have read two blocks");
        assert_eq!(blocks.crypto_blocks.len(), 2);
    }

    #[test]
    fn parse_duplicate_name() {
        let block = r#"name         : deflate
driver       : deflate-generic
module       : kernel
priority     : 0
refcnt       : 2
selftest     : passed
internal     : no
type         : compression

name         : deflate
driver       : deflate-non-generic
module       : kernel
priority     : 0
refcnt       : 2
selftest     : passed
internal     : no
type         : compression
"#;
        let blocks = CryptoTable::from_buf_read(block.as_bytes());
        let blocks = blocks.expect("Should be have read two blocks");
        assert_eq!(blocks.crypto_blocks.len(), 1);
        let deflate_vec = blocks
            .crypto_blocks
            .get("deflate")
            .expect("Should have created a vec of deflates");
        assert_eq!(deflate_vec.len(), 2);
    }

    #[test]
    fn parse_unknown() {
        let block = r#"driver       : ccm_base(ctr(aes-aesni),cbcmac(aes-aesni))
module       : ccm
priority     : 300
refcnt       : 4
selftest     : passed
internal     : no
type         : unknown
key          : val
key2         : val2
"#;
        let mut iter = block.lines().map(|s| Ok(s.to_string())).peekable();
        let block = CryptoBlock::from_iter(&mut iter, "ccm(aes)");
        let block = block.expect("Should be have read one block");
        let mut compare = HashMap::new();
        compare.insert(String::from("key"), String::from("val"));
        compare.insert(String::from("key2"), String::from("val2"));
        compare.insert(String::from("name"), String::from("unknown"));
        assert_eq!(block.crypto_type, Type::Unknown(Unknown { fields: compare }));
    }

    #[test]
    fn parse_unknown_top() {
        let block = r#"name         : ccm(aes)
driver       : ccm_base(ctr(aes-aesni),cbcmac(aes-aesni))
module       : ccm
priority     : 300
refcnt       : 4
selftest     : passed
internal     : no
type         : unknown
key          : val
key2         : val2

name         : ctr(aes)
driver       : ctr(aes-aesni)
module       : kernel
priority     : 300
refcnt       : 4
selftest     : passed
internal     : no
type         : skcipher
async        : no
blocksize    : 1
min keysize  : 16
max keysize  : 32
ivsize       : 16
chunksize    : 16
walksize     : 16
"#;
        let blocks = CryptoTable::from_buf_read(block.as_bytes());
        let blocks = blocks.expect("Should be have read one block");
        assert_eq!(blocks.crypto_blocks.len(), 2);
    }
}
