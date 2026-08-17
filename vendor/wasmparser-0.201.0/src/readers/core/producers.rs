/* Copyright 2019 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{BinaryReader, FromReader, Result, SectionLimited};

/// A reader for the producers custom section of a WebAssembly module.
///
/// # Examples
///
/// ```
/// # let data: &[u8] = &[0x01, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
/// #     0x02, 0x03, 0x77, 0x61, 0x74, 0x01, 0x31, 0x01, 0x43, 0x03, 0x39, 0x2e, 0x30];
/// use wasmparser::{ProducersSectionReader, ProducersFieldValue, Result};
/// let reader = ProducersSectionReader::new(data, 0).expect("producers reader");
/// let field = reader.into_iter().next().unwrap().expect("producers field");
/// assert!(field.name == "language");
/// let value = field.values.into_iter().collect::<Result<Vec<_>>>().expect("values");
/// assert!(value.len() == 2);
/// assert!(value[0].name == "wat" && value[0].version == "1");
/// assert!(value[1].name == "C" && value[1].version == "9.0");
/// ```
pub type ProducersSectionReader<'a> = SectionLimited<'a, ProducersField<'a>>;

/// A field from the producers custom section.
#[derive(Debug, Clone)]
pub struct ProducersField<'a> {
    /// The name of the field.
    pub name: &'a str,
    /// The values specified for this field
    pub values: SectionLimited<'a, ProducersFieldValue<'a>>,
}

impl<'a> FromReader<'a> for ProducersField<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let offset = reader.original_position();
        let name = reader.read_string()?;
        match name {
            "language" | "sdk" | "processed-by" => {}
            _ => bail!(offset, "invalid producers field name: `{name}`"),
        }
        let values = reader.skip(|reader| {
            // FIXME(#188) ideally shouldn't need to skip here
            for _ in 0..reader.read_var_u32()? {
                reader.skip_string()?;
                reader.skip_string()?;
            }
            Ok(())
        })?;
        Ok(ProducersField {
            name,
            values: SectionLimited::new(values.remaining_buffer(), values.original_position())?,
        })
    }
}

/// Represents a field value in the producers custom section.
#[derive(Debug, Copy, Clone)]
pub struct ProducersFieldValue<'a> {
    /// The field name.
    pub name: &'a str,
    /// The field version.
    pub version: &'a str,
}

impl<'a> FromReader<'a> for ProducersFieldValue<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let name = reader.read_string()?;
        let version = reader.read_string()?;
        Ok(ProducersFieldValue { name, version })
    }
}
