#[cfg(feature = "diesel")]
use crate::diesel_ext::bit::BitType;

#[cfg(feature = "diesel")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression};

/// A bit string.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "diesel", diesel(sql_type = BitType))]
pub struct Bit {
    pub(crate) len: usize,
    pub(crate) data: Vec<u8>,
}

impl Bit {
    /// Creates a bit string from a slice of bits.
    pub fn new(data: &[bool]) -> Bit {
        let len = data.len();
        let mut bytes = vec![0; (len + 7) / 8];
        for (i, v) in data.iter().enumerate() {
            bytes[i / 8] |= u8::from(*v) << (7 - (i % 8));
        }
        Bit { len, data: bytes }
    }

    /// Creates a bit string from a slice of bytes.
    pub fn from_bytes(data: &[u8]) -> Bit {
        Bit {
            len: data.len().checked_mul(8).unwrap(),
            data: data.to_vec(),
        }
    }

    /// Returns the number of bits in the bit string.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the bit string is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the bit string as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[cfg(any(feature = "postgres", feature = "sqlx", feature = "diesel"))]
    pub(crate) fn from_sql(buf: &[u8]) -> Result<Bit, Box<dyn std::error::Error + Sync + Send>> {
        let len = i32::from_be_bytes(buf[0..4].try_into()?).try_into()?;
        let data = buf[4..4 + (len + 7) / 8].to_vec();

        Ok(Bit { len, data })
    }
}

#[cfg(test)]
mod tests {
    use crate::Bit;

    #[test]
    fn test_from_bytes() {
        let vec = Bit::from_bytes(&[0b00000000, 0b11111111]);
        assert_eq!(16, vec.len());
        assert_eq!(&[0b00000000, 0b11111111], vec.as_bytes());
    }

    #[test]
    fn test_as_bytes() {
        let vec = Bit::new(&[true, false, true]);
        assert_eq!(3, vec.len());
        assert_eq!(&[0b10100000], vec.as_bytes());
    }

    #[test]
    fn test_is_empty() {
        let vec = Bit::new(&[]);
        assert_eq!(0, vec.len());
        assert!(vec.is_empty());
    }
}
