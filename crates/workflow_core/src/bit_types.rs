use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A single bit - the fundamental unit of all data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bit {
    Zero,
    One,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { Bit::One } else { Bit::Zero }
    }
}

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        matches!(bit, Bit::One)
    }
}

/// A collection of bits with semantic meaning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitVector {
    pub bits: Vec<Bit>,
    pub bit_type: BitType,
}

impl BitVector {
    pub fn new(bits: Vec<Bit>, bit_type: BitType) -> Self {
        Self { bits, bit_type }
    }

    pub fn from_bytes(bytes: &[u8], bit_type: BitType) -> Self {
        let mut bits = Vec::new();
        for byte in bytes {
            for i in 0..8 {
                bits.push(if (byte >> i) & 1 == 1 { Bit::One } else { Bit::Zero });
            }
        }
        Self::new(bits, bit_type)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for chunk in self.bits.chunks(8) {
            let mut byte = 0u8;
            for (i, bit) in chunk.iter().enumerate() {
                if matches!(bit, Bit::One) {
                    byte |= 1 << i;
                }
            }
            bytes.push(byte);
        }
        bytes
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Convert bit vector to integer value
    pub fn to_u64(&self) -> Result<u64, crate::WorkflowError> {
        if self.bits.len() > 64 {
            return Err(crate::WorkflowError::ExecutionError("Bit vector too large for u64".to_string()));
        }
        
        let mut value = 0u64;
        for (i, bit) in self.bits.iter().enumerate() {
            if matches!(bit, Bit::One) {
                value |= 1u64 << i;
            }
        }
        Ok(value)
    }

    /// Create bit vector from integer value
    pub fn from_u64(value: u64, bit_type: BitType) -> Self {
        let bit_count = bit_type.size_in_bits().max(64);
        let mut bits = Vec::new();
        
        for i in 0..bit_count {
            bits.push(if (value >> i) & 1 == 1 { Bit::One } else { Bit::Zero });
        }
        
        Self::new(bits, bit_type)
    }

    /// Add two bit vectors
    pub fn add(&self, other: &BitVector) -> Result<BitVector, crate::WorkflowError> {
        let a = self.to_u64()?;
        let b = other.to_u64()?;
        let result = a.wrapping_add(b);
        
        // Use the larger of the two types for the result
        let result_type = if self.bit_type.size_in_bits() >= other.bit_type.size_in_bits() {
            self.bit_type.clone()
        } else {
            other.bit_type.clone()
        };
        
        Ok(BitVector::from_u64(result, result_type))
    }

    /// Subtract two bit vectors
    pub fn subtract(&self, other: &BitVector) -> Result<BitVector, crate::WorkflowError> {
        let a = self.to_u64()?;
        let b = other.to_u64()?;
        let result = a.wrapping_sub(b);
        
        let result_type = if self.bit_type.size_in_bits() >= other.bit_type.size_in_bits() {
            self.bit_type.clone()
        } else {
            other.bit_type.clone()
        };
        
        Ok(BitVector::from_u64(result, result_type))
    }

    /// Multiply two bit vectors
    pub fn multiply(&self, other: &BitVector) -> Result<BitVector, crate::WorkflowError> {
        let a = self.to_u64()?;
        let b = other.to_u64()?;
        let result = a.wrapping_mul(b);
        
        // For multiplication, use a larger type to accommodate the result
        let result_type = match (&self.bit_type, &other.bit_type) {
            (BitType::Byte, BitType::Byte) => BitType::Word,
            (BitType::Word, BitType::Word) => BitType::DWord,
            (BitType::DWord, BitType::DWord) => BitType::QWord,
            _ => BitType::QWord, // Default to largest type
        };
        
        Ok(BitVector::from_u64(result, result_type))
    }

    /// Divide two bit vectors
    pub fn divide(&self, other: &BitVector) -> Result<(BitVector, BitVector), crate::WorkflowError> {
        let a = self.to_u64()?;
        let b = other.to_u64()?;
        
        if b == 0 {
            return Err(crate::WorkflowError::ExecutionError("Division by zero".to_string()));
        }
        
        let quotient = a / b;
        let remainder = a % b;
        
        let result_type = self.bit_type.clone();
        
        Ok((
            BitVector::from_u64(quotient, result_type.clone()),
            BitVector::from_u64(remainder, result_type)
        ))
    }
}

/// Type information for bit vectors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitType {
    /// Raw bits with no semantic meaning
    Raw,
    /// 8-bit byte
    Byte,
    /// 16-bit word
    Word,
    /// 32-bit double word
    DWord,
    /// 64-bit quad word
    QWord,
    /// UTF-8 encoded text
    Text,
    /// IEEE 754 floating point
    Float32,
    Float64,
    /// Boolean value (single bit)
    Boolean,
    /// Array of another type
    Array(Box<BitType>),
    /// Custom struct type
    Struct(StructType),
}

/// Definition of a custom struct type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub bit_type: BitType,
    pub offset: usize, // Bit offset within the struct
}

impl StructType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, name: String, bit_type: BitType) {
        let offset = self.total_bits();
        self.fields.push(StructField {
            name,
            bit_type,
            offset,
        });
    }

    pub fn total_bits(&self) -> usize {
        self.fields.iter().map(|f| f.bit_type.size_in_bits()).sum()
    }
}

impl BitType {
    /// Get the size of this type in bits
    pub fn size_in_bits(&self) -> usize {
        match self {
            BitType::Raw => 0, // Variable size
            BitType::Byte => 8,
            BitType::Word => 16,
            BitType::DWord => 32,
            BitType::QWord => 64,
            BitType::Text => 0, // Variable size
            BitType::Float32 => 32,
            BitType::Float64 => 64,
            BitType::Boolean => 1,
            BitType::Array(_) => 0, // Variable size
            BitType::Struct(s) => s.total_bits(),
        }
    }

    /// Check if this type is compatible with another for connections
    pub fn is_compatible_with(&self, other: &BitType) -> bool {
        match (self, other) {
            // Raw bits are compatible with anything
            (BitType::Raw, _) | (_, BitType::Raw) => true,
            // Same types are compatible
            (a, b) if a == b => true,
            // Numeric types can be converted between each other
            (BitType::Byte | BitType::Word | BitType::DWord | BitType::QWord | BitType::Float32 | BitType::Float64,
             BitType::Byte | BitType::Word | BitType::DWord | BitType::QWord | BitType::Float32 | BitType::Float64) => true,
            // Arrays are compatible if their element types are compatible
            (BitType::Array(a), BitType::Array(b)) => a.is_compatible_with(b),
            _ => false,
        }
    }
}

impl fmt::Display for BitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitType::Raw => write!(f, "Raw"),
            BitType::Byte => write!(f, "Byte"),
            BitType::Word => write!(f, "Word"),
            BitType::DWord => write!(f, "DWord"),
            BitType::QWord => write!(f, "QWord"),
            BitType::Text => write!(f, "Text"),
            BitType::Float32 => write!(f, "Float32"),
            BitType::Float64 => write!(f, "Float64"),
            BitType::Boolean => write!(f, "Boolean"),
            BitType::Array(inner) => write!(f, "Array<{}>", inner),
            BitType::Struct(s) => write!(f, "Struct({})", s.name),
        }
    }
}

/// A registry for custom struct types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeRegistry {
    pub structs: HashMap<String, StructType>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_struct(&mut self, struct_type: StructType) {
        self.structs.insert(struct_type.name.clone(), struct_type);
    }

    pub fn get_struct(&self, name: &str) -> Option<&StructType> {
        self.structs.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_vector_creation() {
        let bits = vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero];
        let bv = BitVector::new(bits.clone(), BitType::Raw);
        assert_eq!(bv.bits, bits);
        assert_eq!(bv.len(), 4);
    }

    #[test]
    fn test_byte_conversion() {
        let bytes = vec![0b10101010, 0b11110000];
        let bv = BitVector::from_bytes(&bytes, BitType::Raw);
        let converted_back = bv.to_bytes();
        assert_eq!(bytes, converted_back);
    }

    #[test]
    fn test_struct_type() {
        let mut person = StructType::new("Person".to_string());
        person.add_field("age".to_string(), BitType::Byte);
        person.add_field("height".to_string(), BitType::Float32);
        
        assert_eq!(person.total_bits(), 8 + 32);
        assert_eq!(person.fields.len(), 2);
        assert_eq!(person.fields[1].offset, 8);
    }

    #[test]
    fn test_type_compatibility() {
        assert!(BitType::Byte.is_compatible_with(&BitType::Word));
        assert!(BitType::Raw.is_compatible_with(&BitType::Byte));
        assert!(!BitType::Text.is_compatible_with(&BitType::Float32));
    }
} 