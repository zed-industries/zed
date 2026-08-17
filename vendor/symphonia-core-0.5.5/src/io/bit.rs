// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp::min;
use std::io;

use crate::io::ReadBytes;
use crate::util::bits::*;

fn end_of_bitstream_error<T>() -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::Other, "unexpected end of bitstream"))
}

pub mod vlc {
    //! The `vlc` module provides support for decoding variable-length codes (VLC).

    use std::cmp::max;
    use std::collections::{BTreeMap, VecDeque};
    use std::io;

    fn codebook_error<T>(desc: &'static str) -> io::Result<T> {
        Err(io::Error::new(io::ErrorKind::Other, desc))
    }

    /// `BitOrder` describes the relationship between the order of bits in the provided codewords
    /// and the order in which bits are read.
    #[derive(Copy, Clone)]
    pub enum BitOrder {
        /// The provided codewords have bits in the same order as the order in which they're being
        /// read.
        Verbatim,
        /// The provided codeword have bits in the reverse order as the order in which they're
        /// being read.
        Reverse,
    }

    /// `CodebookEntry` provides the functions required for an entry in the `Codebook`.
    pub trait CodebookEntry: Copy + Clone + Default {
        /// The type of a value in this entry.
        type ValueType: Copy + From<u8>;
        /// The type of a jump offset in this entry.
        type OffsetType: Copy;

        /// The maximum jump offset.
        const JUMP_OFFSET_MAX: u32;

        /// Creates a new value entry.
        fn new_value(value: Self::ValueType, len: u8) -> Self;

        /// Create a new jump entry.
        fn new_jump(offset: u32, len: u8) -> Self;

        /// Returns `true` if this entry is a value entry.
        fn is_value(&self) -> bool;

        /// Returns `true` if this entry is a jump entry.
        fn is_jump(&self) -> bool;

        /// Gets the value.
        fn value(&self) -> Self::ValueType;

        /// Get the length of the value in bits.
        fn value_len(&self) -> u32;

        /// Get the position in the table to jump to.
        fn jump_offset(&self) -> usize;

        /// Get the number of bits to read after jumping in the table.
        fn jump_len(&self) -> u32;
    }

    macro_rules! decl_entry {
        (
            #[doc = $expr:expr]
            $name:ident, $value_type:ty, $offset_type:ty, $offset_max:expr, $jump_flag:expr
        ) => {
            #[doc = $expr]
            #[derive(Copy, Clone, Default)]
            pub struct $name($value_type, $offset_type);

            impl CodebookEntry for $name {
                type ValueType = $value_type;
                type OffsetType = $offset_type;

                const JUMP_OFFSET_MAX: u32 = $offset_max;

                #[inline(always)]
                fn new_value(value: Self::ValueType, len: u8) -> Self {
                    $name(value, len.into())
                }

                #[inline(always)]
                fn new_jump(offset: u32, len: u8) -> Self {
                    $name(len.into(), $jump_flag | offset as Self::OffsetType)
                }

                #[inline(always)]
                fn is_jump(&self) -> bool {
                    self.1 & $jump_flag != 0
                }

                #[inline(always)]
                fn is_value(&self) -> bool {
                    self.1 & $jump_flag == 0
                }

                #[inline(always)]
                fn value(&self) -> Self::ValueType {
                    debug_assert!(self.is_value());
                    self.0
                }

                #[inline(always)]
                fn value_len(&self) -> u32 {
                    debug_assert!(self.is_value());
                    (self.1 & (!$jump_flag)).into()
                }

                #[inline(always)]
                fn jump_offset(&self) -> usize {
                    debug_assert!(self.is_jump());
                    (self.1 & (!$jump_flag)) as usize
                }

                #[inline(always)]
                fn jump_len(&self) -> u32 {
                    debug_assert!(self.is_jump());
                    self.0.into()
                }
            }
        };
    }

    decl_entry!(
        /// `Entry8x8` is a codebook entry for 8-bit values with codes up-to 8-bits.
        Entry8x8,
        u8,
        u8,
        0x7f,
        0x80
    );

    decl_entry!(
        /// `Entry8x16` is a codebook entry for 8-bit values with codes up-to 16-bits.
        Entry8x16,
        u8,
        u16,
        0x7fff,
        0x8000
    );

    decl_entry!(
        /// `Entry8x32` is a codebook entry for 8-bit values with codes up-to 32-bits.
        Entry8x32,
        u8,
        u32,
        0x7fff_ffff,
        0x8000_0000
    );

    decl_entry!(
        /// `Entry16x8` is a codebook entry for 16-bit values with codes up-to 8-bits.
        Entry16x8,
        u16,
        u8,
        0x7f,
        0x80
    );

    decl_entry!(
        /// `Entry16x16` is a codebook entry for 16-bit values with codes up-to 16-bits.
        Entry16x16,
        u16,
        u16,
        0x7fff,
        0x8000
    );

    decl_entry!(
        /// `Entry16x32` is a codebook entry for 16-bit values with codes up-to 32-bits.
        Entry16x32,
        u16,
        u32,
        0x7fff_ffff,
        0x8000_0000
    );

    decl_entry!(
        /// `Entry32x8` is a codebook entry for 32-bit values with codes up-to 8-bits.
        Entry32x8,
        u32,
        u8,
        0x7fff,
        0x80
    );

    decl_entry!(
        /// `Entry32x16` is a codebook entry for 32-bit values with codes up-to 16-bits.
        Entry32x16,
        u32,
        u16,
        0x7fff,
        0x8000
    );

    decl_entry!(
        /// `Entry32x32` is a codebook entry for 32-bit values with codes up-to 32-bits.
        Entry32x32,
        u32,
        u32,
        0x7fff_ffff,
        0x8000_0000
    );

    /// `Codebook` is a variable-length code decoding table that may be used to efficiently read
    /// symbols from a source of bits.
    #[derive(Default)]
    pub struct Codebook<E: CodebookEntry> {
        pub table: Vec<E>,
        pub max_code_len: u32,
        pub init_block_len: u32,
    }

    impl<E: CodebookEntry> Codebook<E> {
        /// Returns `true` if the `Codebook` is empty.
        pub fn is_empty(&self) -> bool {
            self.table.is_empty()
        }
    }

    #[derive(Default)]
    struct CodebookValue<E: CodebookEntry> {
        prefix: u16,
        width: u8,
        value: E::ValueType,
    }

    impl<E: CodebookEntry> CodebookValue<E> {
        fn new(prefix: u16, width: u8, value: E::ValueType) -> Self {
            CodebookValue { prefix, width, value }
        }
    }

    #[derive(Default)]
    struct CodebookBlock<E: CodebookEntry> {
        width: u8,
        nodes: BTreeMap<u16, usize>,
        values: Vec<CodebookValue<E>>,
    }

    /// `CodebookBuilder` generates a `Codebook` using a provided codebook specification and
    /// description.
    pub struct CodebookBuilder {
        max_bits_per_block: u8,
        bit_order: BitOrder,
        is_sparse: bool,
    }

    impl CodebookBuilder {
        /// Instantiates a new `CodebookBuilder`.
        ///
        /// The `bit_order` parameter specifies if the codeword bits should be reversed when
        /// constructing the codebook. If the `BitReader` or `BitStream` reading the constructed
        /// codebook reads bits in an order different from the order of the provided codewords,
        /// then this option can be used to make them compatible.
        pub fn new(bit_order: BitOrder) -> Self {
            CodebookBuilder { max_bits_per_block: 4, bit_order, is_sparse: false }
        }

        /// Instantiates a new `CodebookBuilder` for sparse codebooks.
        ///
        /// A sparse codebook is one in which not all codewords are valid. These invalid codewords
        /// are effectively "unused" and have no value. Therefore, it is illegal for a bitstream to
        /// contain the codeword bit pattern.
        ///
        /// Unused codewords are marked by having a length of 0.
        pub fn new_sparse(bit_order: BitOrder) -> Self {
            CodebookBuilder { max_bits_per_block: 4, bit_order, is_sparse: true }
        }

        /// Specify the maximum number of bits that should be consumed from the source at a time.
        /// This value must be within the range 1 <= `max_bits_per_read` <= 16. Values outside of
        /// this range will cause this function to panic. If not provided, a value will be
        /// automatically chosen.
        pub fn bits_per_read(&mut self, max_bits_per_read: u8) -> &mut Self {
            assert!(max_bits_per_read <= 16);
            assert!(max_bits_per_read > 0);
            self.max_bits_per_block = max_bits_per_read;
            self
        }

        fn generate_lut<E: CodebookEntry>(
            bit_order: BitOrder,
            is_sparse: bool,
            blocks: &[CodebookBlock<E>],
        ) -> io::Result<Vec<E>> {
            // The codebook table.
            let mut table = Vec::new();

            let mut queue = VecDeque::new();

            // The computed end of the table given the blocks in the queue.
            let mut table_end = 0u32;

            if !blocks.is_empty() {
                // Start traversal at the first block.
                queue.push_front(0);

                // The first entry in the table is always a jump to the first block.
                let block = &blocks[0];
                table.push(CodebookEntry::new_jump(1, block.width));
                table_end += 1 + (1 << block.width);
            }

            // Traverse the tree in breadth-first order.
            while !queue.is_empty() {
                // Count of the total number of entries added to the table by this block.
                let mut entry_count = 0;

                // Get the block id at the front of the queue.
                let block_id = queue.pop_front().unwrap();

                // Get the block at the front of the queue.
                let block = &blocks[block_id];
                let block_len = 1 << block.width;

                // The starting index of the current block.
                let table_base = table.len();

                // Resize the table to accomodate all entries within the block.
                table.resize(table_base + block_len, Default::default());

                // Push child blocks onto the queue and record the jump entries in the table. Jumps
                // will be in order of increasing prefix because of the implicit sorting provided
                // by BTreeMap, thus traversing a level of the tree left-to-right.
                for (&child_block_prefix, &child_block_id) in block.nodes.iter() {
                    queue.push_back(child_block_id);

                    // The width of the child block in bits.
                    let child_block_width = blocks[child_block_id].width;

                    // Verify the jump offset does not exceed the entry's jump maximum.
                    if table_end > E::JUMP_OFFSET_MAX {
                        return codebook_error("core (io): codebook overflow");
                    }

                    // Determine the offset into the table depending on the bit-order.
                    let offset = match bit_order {
                        BitOrder::Verbatim => child_block_prefix,
                        BitOrder::Reverse => {
                            child_block_prefix.reverse_bits().rotate_left(u32::from(block.width))
                        }
                    } as usize;

                    // Add a jump entry to table.
                    let jump_entry = CodebookEntry::new_jump(table_end, child_block_width);

                    table[table_base + offset] = jump_entry;

                    // Add the length of the child block to the end of the table.
                    table_end += 1 << child_block_width;

                    // Update the entry count.
                    entry_count += 1;
                }

                // Add value entries into the table. If a value has a prefix width less than the
                // block width, then do-not-care bits must added to the end of the prefix to pad it
                // to the block width.
                for value in block.values.iter() {
                    // The number of do-not-care bits to add to the value's prefix.
                    let num_dnc_bits = block.width - value.width;

                    // Extend the value's prefix to the block's width.
                    let base_prefix = (value.prefix << num_dnc_bits) as usize;

                    // Using the base prefix, synthesize all prefixes for this value.
                    let count = 1 << num_dnc_bits;

                    // The value entry that will be duplicated.
                    let value_entry = CodebookEntry::new_value(value.value, value.width);

                    match bit_order {
                        BitOrder::Verbatim => {
                            // For verbatim bit order, the do-not-care bits are in the LSb
                            // position.
                            let start = table_base + base_prefix;
                            let end = start + count;

                            for entry in table[start..end].iter_mut() {
                                *entry = value_entry;
                            }
                        }
                        BitOrder::Reverse => {
                            // For reverse bit order, the do-not-care bits are in the MSb position.
                            let start = base_prefix;
                            let end = start + count;

                            for prefix in start..end {
                                let offset =
                                    prefix.reverse_bits().rotate_left(u32::from(block.width));

                                table[table_base + offset] = value_entry;
                            }
                        }
                    }

                    // Update the entry count.
                    entry_count += count;
                }

                // If the decoding tree is not sparse, the number of entries added to the table
                // should equal the block length if the. It is a fatal error if this is not true.
                if !is_sparse && entry_count != block_len {
                    return codebook_error("core (io): codebook is incomplete");
                }
            }

            Ok(table)
        }

        /// Construct a `Codebook` using the given codewords, their respective lengths, and values.
        ///
        /// This function may fail if the provided codewords do not form a complete VLC tree, or if
        /// the `CodebookEntry` is undersized.
        ///
        /// This function will panic if the number of code words, code lengths, and values differ.
        pub fn make<E: CodebookEntry>(
            &mut self,
            code_words: &[u32],
            code_lens: &[u8],
            values: &[E::ValueType],
        ) -> io::Result<Codebook<E>> {
            assert!(code_words.len() == code_lens.len());
            assert!(code_words.len() == values.len());

            let mut blocks = Vec::<CodebookBlock<E>>::new();

            let mut max_code_len = 0;

            // Only attempt to generate something if there are code words.
            if !code_words.is_empty() {
                let prefix_mask = !(!0 << self.max_bits_per_block);

                // Push a root block.
                blocks.push(Default::default());

                // Populate the tree
                for ((&code, &code_len), &value) in code_words.iter().zip(code_lens).zip(values) {
                    let mut parent_block_id = 0;
                    let mut len = code_len;

                    // A zero length codeword in a spare codebook is allowed, but not in a regular
                    // codebook.
                    if code_len == 0 {
                        if self.is_sparse {
                            continue;
                        }
                        else {
                            return codebook_error("core (io): zero length codeword");
                        }
                    }

                    while len > self.max_bits_per_block {
                        len -= self.max_bits_per_block;

                        let prefix = ((code >> len) & prefix_mask) as u16;

                        // Recurse down the tree.
                        if let Some(&block_id) = blocks[parent_block_id].nodes.get(&prefix) {
                            parent_block_id = block_id;
                        }
                        else {
                            // Add a child block to the parent block.
                            let block_id = blocks.len();

                            let block = &mut blocks[parent_block_id];

                            block.nodes.insert(prefix, block_id);

                            // The parent's block width must accomodate the prefix of the child.
                            // This is always max_bits_per_block bits.
                            block.width = self.max_bits_per_block;

                            // Append the new block.
                            blocks.push(Default::default());

                            parent_block_id = block_id;
                        }
                    }

                    // The final chunk of code bits always has <= max_bits_per_block bits. Obtain
                    // the final prefix.
                    let prefix = code & (prefix_mask >> (self.max_bits_per_block - len));

                    let block = &mut blocks[parent_block_id];

                    // Push the value.
                    block.values.push(CodebookValue::new(prefix as u16, len, value));

                    // Update the block's width.
                    block.width = max(block.width, len);

                    // Update maximum observed codeword.
                    max_code_len = max(max_code_len, code_len);
                }
            }

            // Generate the codebook lookup table.
            let table = CodebookBuilder::generate_lut(self.bit_order, self.is_sparse, &blocks)?;

            // Determine the first block length if skipping the initial jump entry.
            let init_block_len = table.first().map(|block| block.jump_len()).unwrap_or(0);

            Ok(Codebook { table, max_code_len: u32::from(max_code_len), init_block_len })
        }
    }
}

mod private {
    use std::io;

    pub trait FetchBitsLtr {
        /// Discard any remaining bits in the source and fetch new bits.
        fn fetch_bits(&mut self) -> io::Result<()>;

        /// Fetch new bits, and append them after the remaining bits.
        fn fetch_bits_partial(&mut self) -> io::Result<()>;

        /// Get all the bits in the source.
        fn get_bits(&self) -> u64;

        /// Get the number of bits left in the source.
        fn num_bits_left(&self) -> u32;

        /// Consume `num` bits from the source.
        fn consume_bits(&mut self, num: u32);
    }

    pub trait FetchBitsRtl {
        /// Discard any remaining bits in the source and fetch new bits.
        fn fetch_bits(&mut self) -> io::Result<()>;

        /// Fetch new bits, and append them after the remaining bits.
        fn fetch_bits_partial(&mut self) -> io::Result<()>;

        /// Get all the bits in the source.
        fn get_bits(&self) -> u64;

        /// Get the number of bits left in the source.
        fn num_bits_left(&self) -> u32;

        /// Consume `num` bits from the source.
        fn consume_bits(&mut self, num: u32);
    }
}

/// A `FiniteBitStream` is a bit stream that has a known length in bits.
pub trait FiniteBitStream {
    /// Gets the number of bits left unread.
    fn bits_left(&self) -> u64;
}

/// `ReadBitsLtr` reads bits from most-significant to least-significant.
pub trait ReadBitsLtr: private::FetchBitsLtr {
    /// Discards any saved bits and resets the `BitStream` to prepare it for a byte-aligned read.
    #[inline(always)]
    fn realign(&mut self) {
        let skip = self.num_bits_left() & 0x7;
        self.consume_bits(skip);
    }

    /// Ignores the specified number of bits from the stream or returns an error.
    #[inline(always)]
    fn ignore_bits(&mut self, mut num_bits: u32) -> io::Result<()> {
        if num_bits <= self.num_bits_left() {
            self.consume_bits(num_bits);
        }
        else {
            // Consume whole bit caches directly.
            while num_bits > self.num_bits_left() {
                num_bits -= self.num_bits_left();
                self.fetch_bits()?;
            }

            if num_bits > 0 {
                // Shift out in two parts to prevent panicing when num_bits == 64.
                self.consume_bits(num_bits - 1);
                self.consume_bits(1);
            }
        }

        Ok(())
    }

    /// Ignores one bit from the stream or returns an error.
    #[inline(always)]
    fn ignore_bit(&mut self) -> io::Result<()> {
        self.ignore_bits(1)
    }

    /// Read a single bit as a boolean value or returns an error.
    #[inline(always)]
    fn read_bool(&mut self) -> io::Result<bool> {
        if self.num_bits_left() < 1 {
            self.fetch_bits()?;
        }

        let bit = self.get_bits() & (1 << 63) != 0;

        self.consume_bits(1);
        Ok(bit)
    }

    /// Reads and returns a single bit or returns an error.
    #[inline(always)]
    fn read_bit(&mut self) -> io::Result<u32> {
        if self.num_bits_left() < 1 {
            self.fetch_bits()?;
        }

        let bit = self.get_bits() >> 63;

        self.consume_bits(1);

        Ok(bit as u32)
    }

    /// Reads and returns up to 32-bits or returns an error.
    #[inline(always)]
    fn read_bits_leq32(&mut self, mut bit_width: u32) -> io::Result<u32> {
        debug_assert!(bit_width <= u32::BITS);

        // Shift in two 32-bit operations instead of a single 64-bit operation to avoid panicing
        // when bit_width == 0 (and thus shifting right 64-bits). This is preferred to branching
        // the bit_width == 0 case, since reading up-to 32-bits at a time is a hot code-path.
        let mut bits = (self.get_bits() >> u32::BITS) >> (u32::BITS - bit_width);

        while bit_width > self.num_bits_left() {
            bit_width -= self.num_bits_left();

            self.fetch_bits()?;

            // Unlike the first shift, bit_width is always > 0 here so this operation will never
            // shift by > 63 bits.
            bits |= self.get_bits() >> (u64::BITS - bit_width);
        }

        self.consume_bits(bit_width);

        Ok(bits as u32)
    }

    /// Reads up to 32-bits and interprets them as a signed two's complement integer or returns an
    /// error.
    #[inline(always)]
    fn read_bits_leq32_signed(&mut self, bit_width: u32) -> io::Result<i32> {
        let value = self.read_bits_leq32(bit_width)?;
        Ok(sign_extend_leq32_to_i32(value, bit_width))
    }

    /// Reads and returns up to 64-bits or returns an error.
    #[inline(always)]
    fn read_bits_leq64(&mut self, mut bit_width: u32) -> io::Result<u64> {
        debug_assert!(bit_width <= u64::BITS);

        // Hard-code the bit_width == 0 case as it's not possible to handle both the bit_width == 0
        // and bit_width == 64 cases branchlessly. This should be optimized out when bit_width is
        // known at compile time. Since it's generally rare to need to read up-to 64-bits at a time
        // (as oppopsed to 32-bits), this is an acceptable solution.
        if bit_width == 0 {
            Ok(0)
        }
        else {
            // Since bit_width is always > 0, this shift operation is always < 64, and will
            // therefore never panic.
            let mut bits = self.get_bits() >> (u64::BITS - bit_width);

            while bit_width > self.num_bits_left() {
                bit_width -= self.num_bits_left();

                self.fetch_bits()?;

                bits |= self.get_bits() >> (u64::BITS - bit_width);
            }

            // Shift in two parts to prevent panicing when bit_width == 64.
            self.consume_bits(bit_width - 1);
            self.consume_bits(1);

            Ok(bits)
        }
    }

    /// Reads up to 64-bits and interprets them as a signed two's complement integer or returns an
    /// error.
    #[inline(always)]
    fn read_bits_leq64_signed(&mut self, bit_width: u32) -> io::Result<i64> {
        let value = self.read_bits_leq64(bit_width)?;
        Ok(sign_extend_leq64_to_i64(value, bit_width))
    }

    /// Reads and returns a unary zeros encoded integer or an error.
    #[inline(always)]
    fn read_unary_zeros(&mut self) -> io::Result<u32> {
        let mut num = 0;

        loop {
            // Get the number of leading zeros.
            let num_zeros = self.get_bits().leading_zeros();

            if num_zeros >= self.num_bits_left() {
                // If the number of zeros exceeds the number of bits left then all the remaining
                // bits were 0.
                num += self.num_bits_left();
                self.fetch_bits()?;
            }
            else {
                // Otherwise, a 1 bit was encountered after `n_zeros` 0 bits.
                num += num_zeros;

                // Since bits are shifted off the cache after they're consumed, for there to be a
                // 1 bit there must be atleast one extra available bit in the cache that can be
                // consumed after the 0 bits.
                self.consume_bits(num_zeros);
                self.consume_bits(1);

                // Done decoding.
                break;
            }
        }

        Ok(num)
    }

    /// Reads and returns a unary zeros encoded integer that is capped to a maximum value.
    #[inline(always)]
    fn read_unary_zeros_capped(&mut self, mut limit: u32) -> io::Result<u32> {
        let mut num = 0;

        loop {
            // Get the number of leading zeros, capped to the limit.
            let num_bits_left = self.num_bits_left();
            let num_zeros = min(self.get_bits().leading_zeros(), num_bits_left);

            if num_zeros >= limit {
                // There are more ones than the limit. A terminator cannot be encountered.
                num += limit;
                self.consume_bits(limit);
                break;
            }
            else {
                // There are less ones than the limit. A terminator was encountered OR more bits
                // are needed.
                limit -= num_zeros;
                num += num_zeros;

                if num_zeros < num_bits_left {
                    // There are less ones than the number of bits left in the reader. Thus, a
                    // terminator was not encountered and not all bits have not been consumed.
                    self.consume_bits(num_zeros);
                    self.consume_bits(1);
                    break;
                }
            }

            self.fetch_bits()?;
        }

        Ok(num)
    }

    /// Reads and returns a unary ones encoded integer or an error.
    #[inline(always)]
    fn read_unary_ones(&mut self) -> io::Result<u32> {
        // Note: This algorithm is identical to read_unary_zeros except flipped for 1s.
        let mut num = 0;

        loop {
            let num_ones = self.get_bits().leading_ones();

            if num_ones >= self.num_bits_left() {
                num += self.num_bits_left();
                self.fetch_bits()?;
            }
            else {
                num += num_ones;

                self.consume_bits(num_ones);
                self.consume_bits(1);

                break;
            }
        }

        Ok(num)
    }

    /// Reads and returns a unary ones encoded integer that is capped to a maximum value.
    #[inline(always)]
    fn read_unary_ones_capped(&mut self, mut limit: u32) -> io::Result<u32> {
        // Note: This algorithm is identical to read_unary_zeros_capped except flipped for 1s.
        let mut num = 0;

        loop {
            let num_bits_left = self.num_bits_left();
            let num_ones = min(self.get_bits().leading_ones(), num_bits_left);

            if num_ones >= limit {
                num += limit;
                self.consume_bits(limit);
                break;
            }
            else {
                limit -= num_ones;
                num += num_ones;

                if num_ones < num_bits_left {
                    self.consume_bits(num_ones);
                    self.consume_bits(1);
                    break;
                }
            }

            self.fetch_bits()?;
        }

        Ok(num)
    }

    /// Reads a codebook value from the `BitStream` using the provided `Codebook` and returns the
    /// decoded value or an error.
    #[inline(always)]
    fn read_codebook<E: vlc::CodebookEntry>(
        &mut self,
        codebook: &vlc::Codebook<E>,
    ) -> io::Result<(E::ValueType, u32)> {
        // Attempt refill the bit buffer with enough bits for the longest codeword in the codebook.
        // However, this does not mean the bit buffer will have enough bits to decode a codeword.
        if self.num_bits_left() < codebook.max_code_len {
            self.fetch_bits_partial()?;
        }

        // The number of bits actually buffered in the bit buffer.
        let num_bits_left = self.num_bits_left();

        let mut bits = self.get_bits();

        let mut block_len = codebook.init_block_len;
        let mut entry = codebook.table[(bits >> (u64::BITS - block_len)) as usize + 1];

        let mut consumed = 0;

        while entry.is_jump() {
            // Consume the bits used for the initial or previous jump iteration.
            consumed += block_len;
            bits <<= block_len;

            // Since this is a jump entry, if there are no bits left then the bitstream ended early.
            if consumed > num_bits_left {
                return end_of_bitstream_error();
            }

            // Prepare for the next jump.
            block_len = entry.jump_len();

            let index = bits >> (u64::BITS - block_len);

            // Jump to the next entry.
            entry = codebook.table[entry.jump_offset() + index as usize];
        }

        // The entry is always a value entry at this point. Consume the bits containing the value.
        consumed += entry.value_len();

        if consumed > num_bits_left {
            return end_of_bitstream_error();
        }

        self.consume_bits(consumed);

        Ok((entry.value(), consumed))
    }
}

/// `BitStreamLtr` reads bits from most-significant to least-significant from any source
/// that implements [`ReadBytes`].
///
/// Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
/// is the most-significant bit, and bit N-1, the last bit read, is the least-significant.
pub struct BitStreamLtr<'a, B: ReadBytes> {
    reader: &'a mut B,
    bits: u64,
    n_bits_left: u32,
}

impl<'a, B: ReadBytes> BitStreamLtr<'a, B> {
    /// Instantiate a new `BitStreamLtr` with the given source.
    pub fn new(reader: &'a mut B) -> Self {
        BitStreamLtr { reader, bits: 0, n_bits_left: 0 }
    }
}

impl<B: ReadBytes> private::FetchBitsLtr for BitStreamLtr<'_, B> {
    #[inline(always)]
    fn fetch_bits(&mut self) -> io::Result<()> {
        self.bits = u64::from(self.reader.read_u8()?) << 56;
        self.n_bits_left = u8::BITS;
        Ok(())
    }

    #[inline(always)]
    fn fetch_bits_partial(&mut self) -> io::Result<()> {
        todo!()
    }

    #[inline(always)]
    fn get_bits(&self) -> u64 {
        self.bits
    }

    #[inline(always)]
    fn num_bits_left(&self) -> u32 {
        self.n_bits_left
    }

    #[inline(always)]
    fn consume_bits(&mut self, num: u32) {
        self.n_bits_left -= num;
        self.bits <<= num;
    }
}

impl<B: ReadBytes> ReadBitsLtr for BitStreamLtr<'_, B> {}

/// `BitReaderLtr` reads bits from most-significant to least-significant from any `&[u8]`.
///
/// Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
/// is the most-significant bit, and bit N-1, the last bit read, is the least-significant.
pub struct BitReaderLtr<'a> {
    buf: &'a [u8],
    bits: u64,
    n_bits_left: u32,
}

impl<'a> BitReaderLtr<'a> {
    /// Instantiate a new `BitReaderLtr` with the given buffer.
    pub fn new(buf: &'a [u8]) -> Self {
        BitReaderLtr { buf, bits: 0, n_bits_left: 0 }
    }
}

impl private::FetchBitsLtr for BitReaderLtr<'_> {
    #[inline]
    fn fetch_bits_partial(&mut self) -> io::Result<()> {
        let mut buf = [0u8; std::mem::size_of::<u64>()];

        let read_len = min(self.buf.len(), (u64::BITS - self.n_bits_left) as usize >> 3);

        buf[..read_len].copy_from_slice(&self.buf[..read_len]);

        self.buf = &self.buf[read_len..];

        self.bits |= u64::from_be_bytes(buf) >> self.n_bits_left;
        self.n_bits_left += (read_len as u32) << 3;

        Ok(())
    }

    fn fetch_bits(&mut self) -> io::Result<()> {
        let mut buf = [0u8; std::mem::size_of::<u64>()];

        let read_len = min(self.buf.len(), std::mem::size_of::<u64>());

        if read_len == 0 {
            return end_of_bitstream_error();
        }

        buf[..read_len].copy_from_slice(&self.buf[..read_len]);

        self.buf = &self.buf[read_len..];

        self.bits = u64::from_be_bytes(buf);
        self.n_bits_left = (read_len as u32) << 3;

        Ok(())
    }

    #[inline(always)]
    fn get_bits(&self) -> u64 {
        self.bits
    }

    #[inline(always)]
    fn num_bits_left(&self) -> u32 {
        self.n_bits_left
    }

    #[inline(always)]
    fn consume_bits(&mut self, num: u32) {
        self.n_bits_left -= num;
        self.bits <<= num;
    }
}

impl ReadBitsLtr for BitReaderLtr<'_> {}

impl FiniteBitStream for BitReaderLtr<'_> {
    fn bits_left(&self) -> u64 {
        (8 * self.buf.len() as u64) + u64::from(self.n_bits_left)
    }
}

/// `ReadBitsRtl` reads bits from least-significant to most-significant.
pub trait ReadBitsRtl: private::FetchBitsRtl {
    /// Discards any saved bits and resets the `BitStream` to prepare it for a byte-aligned read.
    #[inline(always)]
    fn realign(&mut self) {
        let skip = self.num_bits_left() & 0x7;
        self.consume_bits(skip);
    }

    /// Ignores the specified number of bits from the stream or returns an error.
    #[inline(always)]
    fn ignore_bits(&mut self, mut num_bits: u32) -> io::Result<()> {
        if num_bits <= self.num_bits_left() {
            self.consume_bits(num_bits);
        }
        else {
            // Consume whole bit caches directly.
            while num_bits > self.num_bits_left() {
                num_bits -= self.num_bits_left();
                self.fetch_bits()?;
            }

            if num_bits > 0 {
                // Shift out in two parts to prevent panicing when num_bits == 64.
                self.consume_bits(num_bits - 1);
                self.consume_bits(1);
            }
        }

        Ok(())
    }

    /// Ignores one bit from the stream or returns an error.
    #[inline(always)]
    fn ignore_bit(&mut self) -> io::Result<()> {
        self.ignore_bits(1)
    }

    /// Read a single bit as a boolean value or returns an error.
    #[inline(always)]
    fn read_bool(&mut self) -> io::Result<bool> {
        if self.num_bits_left() < 1 {
            self.fetch_bits()?;
        }

        let bit = (self.get_bits() & 1) == 1;

        self.consume_bits(1);
        Ok(bit)
    }

    /// Reads and returns a single bit or returns an error.
    #[inline(always)]
    fn read_bit(&mut self) -> io::Result<u32> {
        if self.num_bits_left() < 1 {
            self.fetch_bits()?;
        }

        let bit = self.get_bits() & 1;

        self.consume_bits(1);

        Ok(bit as u32)
    }

    /// Reads and returns up to 32-bits or returns an error.
    #[inline(always)]
    fn read_bits_leq32(&mut self, bit_width: u32) -> io::Result<u32> {
        debug_assert!(bit_width <= u32::BITS);

        let mut bits = self.get_bits();
        let mut bits_needed = bit_width;

        while bits_needed > self.num_bits_left() {
            bits_needed -= self.num_bits_left();

            self.fetch_bits()?;

            bits |= self.get_bits() << (bit_width - bits_needed);
        }

        self.consume_bits(bits_needed);

        // Since bit_width is <= 32, this shift will never panic.
        let mask = !(!0 << bit_width);

        Ok((bits & mask) as u32)
    }

    /// Reads up to 32-bits and interprets them as a signed two's complement integer or returns an
    /// error.
    #[inline(always)]
    fn read_bits_leq32_signed(&mut self, bit_width: u32) -> io::Result<i32> {
        let value = self.read_bits_leq32(bit_width)?;
        Ok(sign_extend_leq32_to_i32(value, bit_width))
    }

    /// Reads and returns up to 64-bits or returns an error.
    #[inline(always)]
    fn read_bits_leq64(&mut self, bit_width: u32) -> io::Result<u64> {
        debug_assert!(bit_width <= u64::BITS);

        // Hard-code the bit_width == 0 case as it's not possible to handle both the bit_width == 0
        // and bit_width == 64 cases branchlessly. This should be optimized out when bit_width is
        // known at compile time. Since it's generally rare to need to read up-to 64-bits at a time
        // (as oppopsed to 32-bits), this is an acceptable solution.
        if bit_width == 0 {
            Ok(0)
        }
        else {
            let mut bits = self.get_bits();
            let mut bits_needed = bit_width;

            while bits_needed > self.num_bits_left() {
                bits_needed -= self.num_bits_left();

                self.fetch_bits()?;

                // Since bits_needed will always be > 0, this will never shift by > 63 bits if
                // bit_width == 64 and therefore will never panic.
                bits |= self.get_bits() << (bit_width - bits_needed);
            }

            // Shift in two parts to prevent panicing when bit_width == 64.
            self.consume_bits(bits_needed - 1);
            self.consume_bits(1);

            // Generate the mask in two parts to prevent panicing when bit_width == 64.
            let mask = !((!0 << (bit_width - 1)) << 1);

            Ok(bits & mask)
        }
    }

    /// Reads up to 64-bits and interprets them as a signed two's complement integer or returns an
    /// error.
    #[inline(always)]
    fn read_bits_leq64_signed(&mut self, bit_width: u32) -> io::Result<i64> {
        let value = self.read_bits_leq64(bit_width)?;
        Ok(sign_extend_leq64_to_i64(value, bit_width))
    }

    /// Reads and returns a unary zeros encoded integer or an error.
    #[inline(always)]
    fn read_unary_zeros(&mut self) -> io::Result<u32> {
        let mut num = 0;

        loop {
            // Get the number of trailing zeros.
            let num_zeros = self.get_bits().trailing_zeros();

            if num_zeros >= self.num_bits_left() {
                // If the number of zeros exceeds the number of bits left then all the remaining
                // bits were 0.
                num += self.num_bits_left();
                self.fetch_bits()?;
            }
            else {
                // Otherwise, a 1 bit was encountered after `n_zeros` 0 bits.
                num += num_zeros;

                // Since bits are shifted off the cache after they're consumed, for there to be a
                // 1 bit there must be atleast one extra available bit in the cache that can be
                // consumed after the 0 bits.
                self.consume_bits(num_zeros);
                self.consume_bits(1);

                // Done decoding.
                break;
            }
        }

        Ok(num)
    }

    /// Reads and returns a unary zeros encoded integer that is capped to a maximum value.
    #[inline(always)]
    fn read_unary_zeros_capped(&mut self, mut limit: u32) -> io::Result<u32> {
        let mut num = 0;

        loop {
            // Get the number of trailing zeros, capped to the limit.
            let num_bits_left = self.num_bits_left();
            let num_zeros = min(self.get_bits().trailing_zeros(), num_bits_left);

            if num_zeros >= limit {
                // There are more zeros than the limit. A terminator cannot be encountered.
                num += limit;
                self.consume_bits(limit);
                break;
            }
            else {
                // There are less zeros than the limit. A terminator was encountered OR more bits
                // are needed.
                limit -= num_zeros;
                num += num_zeros;

                if num_zeros < num_bits_left {
                    // There are less zeros than the number of bits left in the reader. Thus, a
                    // terminator was not encountered and not all bits have not been consumed.
                    self.consume_bits(num_zeros);
                    self.consume_bits(1);
                    break;
                }
            }

            self.fetch_bits()?;
        }

        Ok(num)
    }

    /// Reads and returns a unary ones encoded integer or an error.
    #[inline(always)]
    fn read_unary_ones(&mut self) -> io::Result<u32> {
        // Note: This algorithm is identical to read_unary_zeros except flipped for 1s.
        let mut num = 0;

        loop {
            let num_ones = self.get_bits().trailing_ones();

            if num_ones >= self.num_bits_left() {
                num += self.num_bits_left();
                self.fetch_bits()?;
            }
            else {
                num += num_ones;

                self.consume_bits(num_ones);
                self.consume_bits(1);

                break;
            }
        }

        Ok(num)
    }

    /// Reads and returns a unary ones encoded integer or an error.
    #[inline(always)]
    fn read_unary_ones_capped(&mut self, mut limit: u32) -> io::Result<u32> {
        // Note: This algorithm is identical to read_unary_zeros_capped except flipped for 1s.
        let mut num = 0;

        loop {
            let num_bits_left = self.num_bits_left();
            let num_ones = min(self.get_bits().trailing_ones(), num_bits_left);

            if num_ones >= limit {
                num += limit;
                self.consume_bits(limit);
                break;
            }
            else {
                limit -= num_ones;
                num += num_ones;

                if num_ones < num_bits_left {
                    self.consume_bits(num_ones);
                    self.consume_bits(1);
                    break;
                }
            }

            self.fetch_bits()?;
        }

        Ok(num)
    }

    #[inline(always)]
    fn read_codebook<E: vlc::CodebookEntry>(
        &mut self,
        codebook: &vlc::Codebook<E>,
    ) -> io::Result<(E::ValueType, u32)> {
        if self.num_bits_left() < codebook.max_code_len {
            self.fetch_bits_partial()?;
        }

        // The number of bits actually buffered in the bit buffer.
        let num_bits_left = self.num_bits_left();

        let mut bits = self.get_bits();

        let mut block_len = codebook.init_block_len;
        let mut entry = codebook.table[(bits & ((1 << block_len) - 1)) as usize + 1];

        let mut consumed = 0;

        while entry.is_jump() {
            // Consume the bits used for the initial or previous jump iteration.
            consumed += block_len;
            bits >>= block_len;

            // Since this is a jump entry, if there are no bits left then the bitstream ended early.
            if consumed > num_bits_left {
                return end_of_bitstream_error();
            }

            // Prepare for the next jump.
            block_len = entry.jump_len();

            let index = bits & ((1 << block_len) - 1);

            // Jump to the next entry.
            entry = codebook.table[entry.jump_offset() + index as usize];
        }

        // The entry is always a value entry at this point. Consume the bits containing the value.
        consumed += entry.value_len();

        if consumed > num_bits_left {
            return end_of_bitstream_error();
        }

        self.consume_bits(consumed);

        Ok((entry.value(), consumed))
    }
}

/// `BitStreamRtl` reads bits from least-significant to most-significant from any source
/// that implements [`ReadBytes`].
///
/// Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
/// is the least-significant bit, and bit N-1, the last bit read, is the most-significant.
pub struct BitStreamRtl<'a, B: ReadBytes> {
    reader: &'a mut B,
    bits: u64,
    n_bits_left: u32,
}

impl<'a, B: ReadBytes> BitStreamRtl<'a, B> {
    /// Instantiate a new `BitStreamRtl` with the given buffer.
    pub fn new(reader: &'a mut B) -> Self {
        BitStreamRtl { reader, bits: 0, n_bits_left: 0 }
    }
}

impl<B: ReadBytes> private::FetchBitsRtl for BitStreamRtl<'_, B> {
    #[inline(always)]
    fn fetch_bits(&mut self) -> io::Result<()> {
        self.bits = u64::from(self.reader.read_u8()?);
        self.n_bits_left = u8::BITS;
        Ok(())
    }

    #[inline(always)]
    fn fetch_bits_partial(&mut self) -> io::Result<()> {
        todo!()
    }

    #[inline(always)]
    fn get_bits(&self) -> u64 {
        self.bits
    }

    #[inline(always)]
    fn num_bits_left(&self) -> u32 {
        self.n_bits_left
    }

    #[inline(always)]
    fn consume_bits(&mut self, num: u32) {
        self.n_bits_left -= num;
        self.bits >>= num;
    }
}

impl<B: ReadBytes> ReadBitsRtl for BitStreamRtl<'_, B> {}

/// `BitReaderRtl` reads bits from least-significant to most-significant from any `&[u8]`.
///
/// Stated another way, if N-bits are read from a `BitReaderRtl` then bit 0, the first bit read,
/// is the least-significant bit, and bit N-1, the last bit read, is the most-significant.
pub struct BitReaderRtl<'a> {
    buf: &'a [u8],
    bits: u64,
    n_bits_left: u32,
}

impl<'a> BitReaderRtl<'a> {
    /// Instantiate a new `BitReaderRtl` with the given buffer.
    pub fn new(buf: &'a [u8]) -> Self {
        BitReaderRtl { buf, bits: 0, n_bits_left: 0 }
    }
}

impl private::FetchBitsRtl for BitReaderRtl<'_> {
    #[inline]
    fn fetch_bits_partial(&mut self) -> io::Result<()> {
        let mut buf = [0u8; std::mem::size_of::<u64>()];

        let read_len = min(self.buf.len(), (u64::BITS - self.n_bits_left) as usize >> 3);

        buf[..read_len].copy_from_slice(&self.buf[..read_len]);

        self.buf = &self.buf[read_len..];

        self.bits |= u64::from_le_bytes(buf) << self.n_bits_left;
        self.n_bits_left += (read_len as u32) << 3;

        Ok(())
    }

    fn fetch_bits(&mut self) -> io::Result<()> {
        let mut buf = [0u8; std::mem::size_of::<u64>()];

        let read_len = min(self.buf.len(), std::mem::size_of::<u64>());

        if read_len == 0 {
            return end_of_bitstream_error();
        }

        buf[..read_len].copy_from_slice(&self.buf[..read_len]);

        self.buf = &self.buf[read_len..];

        self.bits = u64::from_le_bytes(buf);
        self.n_bits_left = (read_len as u32) << 3;

        Ok(())
    }

    #[inline(always)]
    fn get_bits(&self) -> u64 {
        self.bits
    }

    #[inline(always)]
    fn num_bits_left(&self) -> u32 {
        self.n_bits_left
    }

    #[inline(always)]
    fn consume_bits(&mut self, num: u32) {
        self.n_bits_left -= num;
        self.bits >>= num;
    }
}

impl ReadBitsRtl for BitReaderRtl<'_> {}

impl FiniteBitStream for BitReaderRtl<'_> {
    fn bits_left(&self) -> u64 {
        (8 * self.buf.len() as u64) + u64::from(self.n_bits_left)
    }
}

#[cfg(test)]
mod tests {
    use super::vlc::{BitOrder, Codebook, CodebookBuilder, Entry8x8};
    use super::{BitReaderLtr, ReadBitsLtr};
    use super::{BitReaderRtl, ReadBitsRtl};

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn verify_bitstreamltr_ignore_bits() {
        let mut bs = BitReaderLtr::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xc0, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0a, //
        ]);

        assert_eq!(bs.read_bool().unwrap(), true);

        bs.ignore_bits(128).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);

        bs.ignore_bits(7).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);

        bs.ignore_bits(19).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);

        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);

        bs.ignore_bits(24).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);

        // Lower limit test.
        let mut bs = BitReaderLtr::new(&[0x00]);

        assert!(bs.ignore_bits(0).is_ok());

        let mut bs = BitReaderLtr::new(&[]);

        assert!(bs.ignore_bits(0).is_ok());
        assert!(bs.ignore_bits(1).is_err());

        // Upper limit test.
        let mut bs = BitReaderLtr::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
        ]);

        assert!(bs.ignore_bits(64).is_ok());
        assert!(bs.ignore_bits(64).is_ok());
        assert!(bs.ignore_bits(32).is_ok());
        assert!(bs.ignore_bits(32).is_ok());
        assert!(bs.ignore_bits(64).is_ok());
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn verify_bitstreamltr_read_bool() {
        // General tests.
        let mut bs = BitReaderLtr::new(&[0b1010_1010]);

        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);

        // Error test.
        let mut bs = BitReaderLtr::new(&[]);

        assert!(bs.read_bool().is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_bit() {
        // General tests.
        let mut bs = BitReaderLtr::new(&[0b1010_1010]);

        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);

        // Error test.
        let mut bs = BitReaderLtr::new(&[]);

        assert!(bs.read_bool().is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_bits_leq32() {
        // General tests.
        let mut bs = BitReaderLtr::new(&[0b1010_0101, 0b0111_1110, 0b1101_0011]);

        assert_eq!(bs.read_bits_leq32(4).unwrap(), 0b0000_0000_0000_1010);
        assert_eq!(bs.read_bits_leq32(4).unwrap(), 0b0000_0000_0000_0101);
        assert_eq!(bs.read_bits_leq32(13).unwrap(), 0b0000_1111_1101_1010);
        assert_eq!(bs.read_bits_leq32(3).unwrap(), 0b0000_0000_0000_0011);

        // Lower limit test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff]);

        assert_eq!(bs.read_bits_leq32(0).unwrap(), 0);

        // Upper limit test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(8).unwrap(), 0x01);

        // Cache fetch test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(8).unwrap(), 0x01);

        // Test error cases.
        let mut bs = BitReaderLtr::new(&[0xff]);

        assert!(bs.read_bits_leq32(9).is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_bits_leq64() {
        // General tests.
        let mut bs = BitReaderLtr::new(&[
            0x99, 0xaa, 0x55, 0xff, 0xff, 0x55, 0xaa, 0x99, //
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, //
        ]);

        assert_eq!(bs.read_bits_leq64(40).unwrap(), 0x99aa55ffff);
        assert_eq!(bs.read_bits_leq64(4).unwrap(), 0x05);
        assert_eq!(bs.read_bits_leq64(4).unwrap(), 0x05);
        assert_eq!(bs.read_bits_leq64(16).unwrap(), 0xaa99);
        assert_eq!(bs.read_bits_leq64(64).unwrap(), 0x1122334455667788);

        // Lower limit test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        assert_eq!(bs.read_bits_leq64(0).unwrap(), 0);

        // Upper limit test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq64(64).unwrap(), u64::MAX);
        assert_eq!(bs.read_bits_leq64(8).unwrap(), 0x01);

        // Test error cases.
        let mut bs = BitReaderLtr::new(&[0xff]);

        assert!(bs.read_bits_leq64(9).is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_unary_zeros() {
        // General tests
        let mut bs =
            BitReaderLtr::new(&[0b0000_0001, 0b0001_0000, 0b0000_0000, 0b1000_0000, 0b1111_1011]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 7);
        assert_eq!(bs.read_unary_zeros().unwrap(), 3);
        assert_eq!(bs.read_unary_zeros().unwrap(), 12);
        assert_eq!(bs.read_unary_zeros().unwrap(), 7);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 1);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);

        // Upper limit test
        let mut bs = BitReaderLtr::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 63);

        // Lower limit test
        let mut bs = BitReaderLtr::new(&[0x80]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 0);

        // Error test.
        let mut bs = BitReaderLtr::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        assert!(bs.read_unary_zeros().is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_unary_zeros_capped() {
        // Basic test
        let mut bs = BitReaderLtr::new(&[0b0000_0001, 0b0000_0001]);

        assert_eq!(bs.read_unary_zeros_capped(8).unwrap(), 7);
        assert_eq!(bs.read_unary_zeros_capped(4).unwrap(), 4);

        // Long limit test
        let mut bs = BitReaderLtr::new(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        ]);

        assert_eq!(bs.read_unary_zeros_capped(96).unwrap(), 79);
        assert_eq!(bs.read_unary_zeros_capped(104).unwrap(), 104);
    }

    #[test]
    fn verify_bitstreamltr_read_unary_ones() {
        // General tests
        let mut bs =
            BitReaderLtr::new(&[0b1111_1110, 0b1110_1111, 0b1111_1111, 0b0111_1111, 0b0000_0100]);

        assert_eq!(bs.read_unary_ones().unwrap(), 7);
        assert_eq!(bs.read_unary_ones().unwrap(), 3);
        assert_eq!(bs.read_unary_ones().unwrap(), 12);
        assert_eq!(bs.read_unary_ones().unwrap(), 7);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 1);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);

        // Upper limit test
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe]);

        assert_eq!(bs.read_unary_ones().unwrap(), 63);

        // Lower limit test
        let mut bs = BitReaderLtr::new(&[0x7f]);

        assert_eq!(bs.read_unary_ones().unwrap(), 0);

        // Error test.
        let mut bs = BitReaderLtr::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        assert!(bs.read_unary_ones().is_err());
    }

    #[test]
    fn verify_bitstreamltr_read_unary_ones_capped() {
        // Basic test
        let mut bs = BitReaderLtr::new(&[0b1111_1110, 0b1111_1110]);

        assert_eq!(bs.read_unary_ones_capped(8).unwrap(), 7);
        assert_eq!(bs.read_unary_ones_capped(4).unwrap(), 4);

        let mut bs =
            BitReaderLtr::new(&[0b1111_1110, 0b1110_1111, 0b1111_1111, 0b0111_1111, 0b0000_0100]);

        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 7);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 3);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 9); // Limit
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 3);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 7);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 0);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 0);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 0);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 0);
        assert_eq!(bs.read_unary_ones_capped(9).unwrap(), 1);

        // Long limit test
        let mut bs = BitReaderLtr::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]);

        assert_eq!(bs.read_unary_ones_capped(144).unwrap(), 143);
        assert_eq!(bs.read_unary_ones_capped(256).unwrap(), 256);
    }

    fn generate_codebook(bit_order: BitOrder) -> (Codebook<Entry8x8>, Vec<u8>, &'static str) {
        // Codewords in MSb bit-order.
        #[rustfmt::skip]
        const CODE_WORDS: [u32; 25] = [
            0b001,
            0b111,
            0b010,
            0b1001,
            0b1101,
            0b0001,
            0b0111,
            0b1000,
            0b10110,
            0b10111,
            0b10101,
            0b10100,
            0b01100,
            0b000010,
            0b110000,
            0b000011,
            0b110001,
            0b000001,
            0b011010,
            0b000000,
            0b011011,
            0b1100111,
            0b1100110,
            0b1100101,
            0b1100100,
        ];

        const CODE_LENS: [u8; 25] =
            [3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7];

        const VALUES: [u8; 25] = [
            b'i', b' ', b'e', b't', b's', b'l', b'n', b'o', b'.', b'r', b'g', b'h', b'u', b'p',
            b'w', b',', b'f', b'y', b'm', b'v', b'a', b'd', b'b', b'c', b'T',
        ];

        // Encoded data in MSb bit-order.
        const DATA: [u8; 57] = [
            0xc9, 0x43, 0xbf, 0x48, 0xa7, 0xca, 0xbe, 0x64, 0x30, 0xf5, 0xdf, 0x31, 0xd9, 0xb6,
            0xb5, 0xbb, 0x6f, 0x9f, 0xa0, 0x15, 0xc1, 0xfa, 0x5e, 0xa2, 0xb8, 0x4a, 0xfb, 0x0f,
            0xe1, 0x93, 0xe6, 0x8a, 0xe8, 0x3e, 0x77, 0xe0, 0xd9, 0x92, 0xf5, 0xf8, 0xc5, 0xfb,
            0x37, 0xcc, 0x7c, 0x48, 0x8f, 0x33, 0xf0, 0x33, 0x4f, 0xb0, 0xd2, 0x9a, 0x17, 0xad,
            0x80,
        ];

        const TEXT: &str = "This silence belongs to us... and every single person out \
                            there, is waiting for us to fill it with something.";

        // Reverse the bits in the data vector if testing a reverse bit-order.
        let data = match bit_order {
            BitOrder::Verbatim => DATA.to_vec(),
            BitOrder::Reverse => DATA.iter().map(|&b| b.reverse_bits()).collect(),
        };

        // Construct a codebook using the tables above.
        let mut builder = CodebookBuilder::new(bit_order);
        let codebook = builder.make::<Entry8x8>(&CODE_WORDS, &CODE_LENS, &VALUES).unwrap();

        (codebook, data, TEXT)
    }

    #[test]
    fn verify_bitstreamltr_read_codebook() {
        let (codebook, buf, text) = generate_codebook(BitOrder::Verbatim);

        let mut bs = BitReaderLtr::new(&buf);

        let decoded: Vec<u8> =
            (0..text.len()).map(|_| bs.read_codebook(&codebook).unwrap().0).collect();

        assert_eq!(text, std::str::from_utf8(&decoded).unwrap());
    }

    // BitStreamRtl

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn verify_bitstreamrtl_ignore_bits() {
        let mut bs = BitReaderRtl::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0x02, 0x08, 0x00, 0x80, 0x00, 0x00, 0x00, 0x50, //
        ]);

        assert_eq!(bs.read_bool().unwrap(), true);

        bs.ignore_bits(128).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);

        bs.ignore_bits(7).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);

        bs.ignore_bits(19).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);

        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), false);

        bs.ignore_bits(24).unwrap();

        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);

        // Lower limit test.
        let mut bs = BitReaderRtl::new(&[0x00]);

        assert!(bs.ignore_bits(0).is_ok());

        let mut bs = BitReaderRtl::new(&[]);

        assert!(bs.ignore_bits(0).is_ok());
        assert!(bs.ignore_bits(1).is_err());

        // Upper limit test.
        let mut bs = BitReaderRtl::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
        ]);

        assert!(bs.ignore_bits(64).is_ok());
        assert!(bs.ignore_bits(64).is_ok());
        assert!(bs.ignore_bits(32).is_ok());
        assert!(bs.ignore_bits(32).is_ok());
        assert!(bs.ignore_bits(64).is_ok());
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn verify_bitstreamrtl_read_bool() {
        // General tests.
        let mut bs = BitReaderRtl::new(&[0b1010_1010]);

        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);
        assert_eq!(bs.read_bool().unwrap(), false);
        assert_eq!(bs.read_bool().unwrap(), true);

        // Error test.
        let mut bs = BitReaderRtl::new(&[]);

        assert!(bs.read_bool().is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_bit() {
        // General tests.
        let mut bs = BitReaderRtl::new(&[0b1010_1010]);

        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);
        assert_eq!(bs.read_bit().unwrap(), 0);
        assert_eq!(bs.read_bit().unwrap(), 1);

        // Error test.
        let mut bs = BitReaderRtl::new(&[]);

        assert!(bs.read_bit().is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_bits_leq32() {
        // General tests.
        let mut bs = BitReaderRtl::new(&[0b1010_0101, 0b0111_1110, 0b1101_0011]);

        assert_eq!(bs.read_bits_leq32(4).unwrap(), 0b0000_0000_0000_0101);
        assert_eq!(bs.read_bits_leq32(4).unwrap(), 0b0000_0000_0000_1010);
        assert_eq!(bs.read_bits_leq32(13).unwrap(), 0b0001_0011_0111_1110);
        assert_eq!(bs.read_bits_leq32(3).unwrap(), 0b0000_0000_0000_0110);

        // Lower limit test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff]);

        assert_eq!(bs.read_bits_leq32(0).unwrap(), 0);

        // Upper limit test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(8).unwrap(), 0x01);

        // Cache fetch test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(32).unwrap(), u32::MAX);
        assert_eq!(bs.read_bits_leq32(8).unwrap(), 0x01);

        // Test error cases.
        let mut bs = BitReaderRtl::new(&[0xff]);

        assert!(bs.read_bits_leq32(9).is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_bits_leq64() {
        // General tests.
        let mut bs = BitReaderRtl::new(&[
            0x99, 0xaa, 0x55, 0xff, 0xff, 0x55, 0xaa, 0x99, //
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, //
            0x00, 0x11, 0x22, 0x33, 0x00, 0x11, 0x22, 0x33, //
            0x44, 0x55, 0x66, 0x77,
        ]);

        assert_eq!(bs.read_bits_leq64(40).unwrap(), 0xffff55aa99);
        assert_eq!(bs.read_bits_leq64(4).unwrap(), 0x05);
        assert_eq!(bs.read_bits_leq64(4).unwrap(), 0x05);
        assert_eq!(bs.read_bits_leq64(16).unwrap(), 0x99aa);
        assert_eq!(bs.read_bits_leq64(64).unwrap(), 0x8877665544332211);
        assert_eq!(bs.read_bits_leq64(32).unwrap(), 0x33221100);
        assert_eq!(bs.read_bits_leq64(64).unwrap(), 0x7766554433221100);

        // Lower limit test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        assert_eq!(bs.read_bits_leq64(0).unwrap(), 0);

        // Upper limit test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

        assert_eq!(bs.read_bits_leq64(64).unwrap(), u64::MAX);
        assert_eq!(bs.read_bits_leq64(8).unwrap(), 0x01);

        // Test error cases.
        let mut bs = BitReaderRtl::new(&[0xff]);

        assert!(bs.read_bits_leq64(9).is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_unary_zeros() {
        // General tests
        let mut bs =
            BitReaderRtl::new(&[0b1000_0000, 0b0000_1000, 0b0000_0000, 0b0000_0001, 0b1101_1111]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 7);
        assert_eq!(bs.read_unary_zeros().unwrap(), 3);
        assert_eq!(bs.read_unary_zeros().unwrap(), 12);
        assert_eq!(bs.read_unary_zeros().unwrap(), 7);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);
        assert_eq!(bs.read_unary_zeros().unwrap(), 1);
        assert_eq!(bs.read_unary_zeros().unwrap(), 0);

        // Upper limit test
        let mut bs = BitReaderRtl::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 63);

        // Lower limit test
        let mut bs = BitReaderRtl::new(&[0x01]);

        assert_eq!(bs.read_unary_zeros().unwrap(), 0);

        // Error test.
        let mut bs = BitReaderRtl::new(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        assert!(bs.read_unary_zeros().is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_unary_zeros_capped() {
        // General tests
        let mut bs = BitReaderRtl::new(&[0b1000_0000, 0b1000_0000]);

        assert_eq!(bs.read_unary_zeros_capped(8).unwrap(), 7);
        assert_eq!(bs.read_unary_zeros_capped(4).unwrap(), 4);

        // Long limit tests
        let mut bs = BitReaderRtl::new(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        ]);

        assert_eq!(bs.read_unary_zeros_capped(96).unwrap(), 79);
        assert_eq!(bs.read_unary_zeros_capped(163).unwrap(), 163);
    }

    #[test]
    fn verify_bitstreamrtl_read_unary_ones() {
        // General tests
        let mut bs =
            BitReaderRtl::new(&[0b0111_1111, 0b1111_0111, 0b1111_1111, 0b1111_1110, 0b0010_0000]);

        assert_eq!(bs.read_unary_ones().unwrap(), 7);
        assert_eq!(bs.read_unary_ones().unwrap(), 3);
        assert_eq!(bs.read_unary_ones().unwrap(), 12);
        assert_eq!(bs.read_unary_ones().unwrap(), 7);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);
        assert_eq!(bs.read_unary_ones().unwrap(), 1);
        assert_eq!(bs.read_unary_ones().unwrap(), 0);

        // Upper limit test
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);

        assert_eq!(bs.read_unary_ones().unwrap(), 63);

        // Lower limit test
        let mut bs = BitReaderRtl::new(&[0xfe]);

        assert_eq!(bs.read_unary_ones().unwrap(), 0);

        // Error test.
        let mut bs = BitReaderRtl::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        assert!(bs.read_unary_ones().is_err());
    }

    #[test]
    fn verify_bitstreamrtl_read_unary_ones_capped() {
        // General tests
        let mut bs = BitReaderRtl::new(&[0b0111_1111, 0b0111_1111]);

        assert_eq!(bs.read_unary_ones_capped(8).unwrap(), 7);
        assert_eq!(bs.read_unary_ones_capped(4).unwrap(), 4);

        // Long limit tests
        let mut bs = BitReaderRtl::new(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
        ]);

        assert_eq!(bs.read_unary_ones_capped(96).unwrap(), 79);
        assert_eq!(bs.read_unary_ones_capped(163).unwrap(), 163);
    }

    #[test]
    fn verify_bitstreamrtl_read_codebook() {
        // The codewords are in MSb bit-order, but reading the bitstream in LSb order. Therefore,
        // use the reverse bit order.
        let (codebook, buf, text) = generate_codebook(BitOrder::Reverse);

        let mut bs = BitReaderRtl::new(&buf);

        let decoded: Vec<u8> =
            (0..text.len()).map(|_| bs.read_codebook(&codebook).unwrap().0).collect();

        assert_eq!(text, std::str::from_utf8(&decoded).unwrap());
    }
}
