// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};

/// An attribute host compatibility supported by this crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeCompatibility {
    Unix,
}

impl TryFrom<u16> for AttributeCompatibility {
    type Error = ZipError;

    // Convert a u16 stored with little endianness into a supported attribute host compatibility.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4422
    fn try_from(value: u16) -> Result<Self> {
        match value {
            3 => Ok(AttributeCompatibility::Unix),
            _ => Err(ZipError::AttributeCompatibilityNotSupported(value)),
        }
    }
}

impl From<&AttributeCompatibility> for u16 {
    // Convert a supported attribute host compatibility into its relevant u16 stored with little endianness.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4422
    fn from(compatibility: &AttributeCompatibility) -> Self {
        match compatibility {
            AttributeCompatibility::Unix => 3,
        }
    }
}

impl From<AttributeCompatibility> for u16 {
    // Convert a supported attribute host compatibility into its relevant u16 stored with little endianness.
    fn from(compatibility: AttributeCompatibility) -> Self {
        (&compatibility).into()
    }
}
