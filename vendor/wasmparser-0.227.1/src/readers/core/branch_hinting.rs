use crate::{BinaryReader, FromReader, Result, SectionLimited};

/// A reader for the `metadata.code.branch_hint` custom section.
pub type BranchHintSectionReader<'a> = SectionLimited<'a, BranchHintFunction<'a>>;

/// Branch hints for a single function.
///
/// Produced from [`BranchHintSectionReader`].
#[derive(Debug, Clone)]
pub struct BranchHintFunction<'a> {
    /// The function that these branch hints apply to.
    pub func: u32,
    /// The branch hints available for this function.
    pub hints: SectionLimited<'a, BranchHint>,
}

impl<'a> FromReader<'a> for BranchHintFunction<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let func = reader.read_var_u32()?;
        // FIXME(#188) ideally wouldn't have to do skips here
        let hints = reader.skip(|reader| {
            let items_count = reader.read_var_u32()?;
            for _ in 0..items_count {
                reader.read::<BranchHint>()?;
            }
            Ok(())
        })?;
        Ok(BranchHintFunction {
            func,
            hints: SectionLimited::new(hints)?,
        })
    }
}

/// A hint for a single branch.
#[derive(Debug, Copy, Clone)]
pub struct BranchHint {
    /// The byte offset, from the start of the function's body, of where the
    /// hinted instruction lives.
    pub func_offset: u32,
    /// Whether or not the branch is hinted to be taken or not.
    pub taken: bool,
}

impl<'a> FromReader<'a> for BranchHint {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let func_offset = reader.read_var_u32()?;
        match reader.read_u8()? {
            1 => {}
            n => reader.invalid_leading_byte(n, "invalid branch hint byte")?,
        }
        let taken = match reader.read_u8()? {
            0 => false,
            1 => true,
            n => reader.invalid_leading_byte(n, "invalid branch hint taken byte")?,
        };
        Ok(BranchHint { func_offset, taken })
    }
}
