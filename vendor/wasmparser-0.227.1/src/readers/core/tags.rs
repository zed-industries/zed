/* Copyright 2020 Mozilla Foundation
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

use crate::{BinaryReader, FromReader, Result, SectionLimited, TagKind, TagType};

/// A reader for the tags section of a WebAssembly module.
pub type TagSectionReader<'a> = SectionLimited<'a, TagType>;

impl<'a> FromReader<'a> for TagType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let attribute = reader.read_u8()?;
        if attribute != 0 {
            bail!(reader.original_position() - 1, "invalid tag attributes");
        }
        Ok(TagType {
            kind: TagKind::Exception,
            func_type_idx: reader.read_var_u32()?,
        })
    }
}
