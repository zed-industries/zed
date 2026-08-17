#[path = "src/tables.rs"]
mod tables;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use block::{Block, LAST_INDEX};
use tables::{GeneralCategory, GENERAL_CATEGORY};

const SHIFT: u32 = block::LAST_INDEX.count_ones();

fn main() {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("category.rs");

    write_table(&output_path, &compile_table());
}

struct CompiledTable {
    blocks: Vec<(u32, Block)>,
    address_to_block_index: Vec<(u32, usize)>,
    last_code_point: u32,
}

fn compile_table() -> CompiledTable {
    let mut blocks = Vec::new();
    let mut address_to_block_index = Vec::new();

    let &(start, _, _) = GENERAL_CATEGORY
        .iter()
        .min_by_key(|(start, _, _)| start)
        .unwrap();
    let &(_, end, _) = GENERAL_CATEGORY
        .iter()
        .max_by_key(|(_, end, _)| end)
        .unwrap();
    let last_code_point = end;

    // Extend end to the end of the last block to ensure the last block is written out
    let end_block_address = end & (!LAST_INDEX as u32);
    let end = end_block_address + block::SIZE as u32;

    let mut block = Block::new();
    for codepoint in start..=end {
        let category = lookup(codepoint);
        let block_address = (codepoint >> SHIFT).saturating_sub(1) << SHIFT;

        // This is the first codepoint in this block, write out the previous block
        if codepoint != 0 && (codepoint & u32::try_from(block::LAST_INDEX).unwrap()) == 0 {
            if let Some(index) = blocks.iter().position(|(_, candidate)| candidate == &block) {
                address_to_block_index.push((block_address, index));
            } else {
                // Add the block if it's new
                address_to_block_index.push((block_address, blocks.len()));
                blocks.push((block_address, block.clone()));
            }

            block.reset();
        }

        block[usize::try_from(codepoint).unwrap() & block::LAST_INDEX] = category;
    }

    CompiledTable {
        blocks,
        address_to_block_index,
        last_code_point,
    }
}

fn write_table(path: &Path, compiled_table: &CompiledTable) {
    let mut output =
        File::create(path).unwrap_or_else(|_| panic!("unable to open {}", path.to_string_lossy()));

    writeln!(output, "use crate::GeneralCategory;").unwrap();
    writeln!(output, "use crate::GeneralCategory::*;").unwrap();

    writeln!(
        output,
        "\nconst LAST_CODEPOINT: u32 = 0x{:X};",
        compiled_table.last_code_point
    )
    .unwrap();
    writeln!(output, "\nconst BLOCK_SIZE: usize = {};", block::SIZE).unwrap();

    // Write out the blocks in address order
    writeln!(
        output,
        "\n#[allow(clippy::large_const_arrays)]\nconst CATEGORY_BLOCKS: [GeneralCategory; {}] = [",
        compiled_table.blocks.len() * block::SIZE
    )
    .unwrap();

    for (address, block) in &compiled_table.blocks {
        writeln!(output, "// BLOCK: {:04X}\n", address).unwrap();
        for (i, category) in block.iter().enumerate() {
            if i != 0 && (i & 0xF) == 0 {
                writeln!(output).unwrap();
            }

            write!(output, "{:?},", category).unwrap();
        }

        write!(output, "\n\n").unwrap();
    }
    writeln!(output, "];").unwrap();

    write!(output, "\n\n").unwrap();

    // Write out constants for the block offsets
    for (index, (address, _)) in compiled_table.blocks.iter().enumerate() {
        writeln!(
            output,
            "const BLOCK_OFFSET_{:04X}: u16 = 0x{:04X};",
            address,
            index * block::SIZE
        )
        .unwrap();
    }

    // Write out the array that maps categories to offsets
    writeln!(
        output,
        "\nconst CATEGORY_BLOCK_OFFSETS: [u16; {}] = [",
        compiled_table.address_to_block_index.len()
    )
    .unwrap();
    for &(_, index) in &compiled_table.address_to_block_index {
        let (block_address, _) = compiled_table.blocks[index];
        writeln!(output, "    BLOCK_OFFSET_{:04X},", block_address).unwrap();
    }
    writeln!(output, "];").unwrap();
}

/// Lookup this code point in the GENERAL_CATEGORY table
fn lookup(codepoint: u32) -> GeneralCategory {
    GENERAL_CATEGORY
        .binary_search_by(|&(start, end, _)| {
            if codepoint < start {
                Ordering::Greater
            } else if codepoint > end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .ok()
        .map(|idx| GENERAL_CATEGORY[idx].2)
        .unwrap_or(GeneralCategory::PrivateUse)
}

mod block {
    pub const SIZE: usize = 256;
    pub const LAST_INDEX: usize = SIZE - 1;

    use super::GeneralCategory;
    use std::ops::{Index, IndexMut};

    /// A fixed size block
    ///
    /// Ideally this would be an array but that doesn't work until const generics are stabilised
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct Block {
        data: Vec<GeneralCategory>,
    }

    impl Block {
        pub fn new() -> Self {
            Block {
                data: vec![GeneralCategory::Unassigned; SIZE],
            }
        }

        pub fn reset(&mut self) {
            self.data
                .iter_mut()
                .for_each(|val| *val = GeneralCategory::Unassigned);
        }

        pub fn iter(&self) -> impl Iterator<Item = &GeneralCategory> {
            self.data.iter()
        }
    }

    impl Index<usize> for Block {
        type Output = GeneralCategory;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexMut<usize> for Block {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.data.index_mut(index)
        }
    }
}
