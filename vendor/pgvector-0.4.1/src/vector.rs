#[cfg(any(feature = "postgres", feature = "sqlx", feature = "diesel"))]
use std::convert::TryInto;

#[cfg(feature = "diesel")]
use crate::diesel_ext::vector::VectorType;

#[cfg(feature = "diesel")]
use diesel::{deserialize::FromSqlRow, expression::AsExpression};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A vector.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "diesel", diesel(sql_type = VectorType))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Vector(pub(crate) Vec<f32>);

impl From<Vec<f32>> for Vector {
    fn from(v: Vec<f32>) -> Self {
        Vector(v)
    }
}

impl From<Vector> for Vec<f32> {
    fn from(val: Vector) -> Self {
        val.0
    }
}

impl Vector {
    /// Returns a copy of the vector as a `Vec<f32>`.
    pub fn to_vec(&self) -> Vec<f32> {
        self.0.clone()
    }

    /// Returns the vector as a slice.
    pub fn as_slice(&self) -> &[f32] {
        self.0.as_slice()
    }

    #[cfg(any(feature = "postgres", feature = "sqlx", feature = "diesel"))]
    pub(crate) fn from_sql(buf: &[u8]) -> Result<Vector, Box<dyn std::error::Error + Sync + Send>> {
        let dim = u16::from_be_bytes(buf[0..2].try_into()?).into();
        let unused = u16::from_be_bytes(buf[2..4].try_into()?);
        if unused != 0 {
            return Err("expected unused to be 0".into());
        }

        let mut vec = Vec::with_capacity(dim);
        for i in 0..dim {
            let s = 4 + 4 * i;
            vec.push(f32::from_be_bytes(buf[s..s + 4].try_into()?));
        }

        Ok(Vector(vec))
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_into() {
        let vec = Vector::from(vec![1.0, 2.0, 3.0]);
        let f32_vec: Vec<f32> = vec.into();
        assert_eq!(f32_vec, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_to_vec() {
        let vec = Vector::from(vec![1.0, 2.0, 3.0]);
        assert_eq!(vec.to_vec(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_as_slice() {
        let vec = Vector::from(vec![1.0, 2.0, 3.0]);
        assert_eq!(vec.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        let vec = Vector::from(vec![1.0, 2.0, 3.0]);
        let json = serde_json::to_string(&vec).unwrap();
        assert_eq!(json, "[1.0,2.0,3.0]");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        let json = "[1.0,2.0,3.0]";
        let vec: Vector = serde_json::from_str(json).unwrap();
        assert_eq!(vec, Vector::from(vec![1.0, 2.0, 3.0]));
    }
}
