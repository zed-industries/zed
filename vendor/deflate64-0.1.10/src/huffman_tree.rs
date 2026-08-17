use crate::input_buffer::InputBuffer;
use crate::InternalErr;

#[derive(Debug)]
pub(crate) struct HuffmanTree {
    code_lengths_length: u16,
    table: [i16; 1 << Self::TABLE_BITS],
    left: [i16; Self::MAX_CODE_LENGTHS * 2],
    right: [i16; Self::MAX_CODE_LENGTHS * 2],
    code_length_array: [u8; Self::MAX_CODE_LENGTHS],
}

// because of lifetime conflict, we cannot use simple accessor method.
macro_rules! get {
    ($self: ident.table) => {
        $self.table[..]
    };
    ($self: ident.left) => {
        $self.left[..2 * $self.code_lengths_length as usize]
    };
    ($self: ident.right) => {
        $self.right[..2 * $self.code_lengths_length as usize]
    };
    ($self: ident.code_length_array) => {
        $self.code_length_array[..$self.code_lengths_length as usize]
    };
}

impl HuffmanTree {
    pub(crate) const MAX_CODE_LENGTHS: usize = 288;
    pub(crate) const TABLE_BITS: u8 = 9;
    pub(crate) const TABLE_BITS_MASK: usize = (1 << Self::TABLE_BITS) - 1;

    pub(crate) const MAX_LITERAL_TREE_ELEMENTS: usize = 288;
    pub(crate) const MAX_DIST_TREE_ELEMENTS: usize = 32;
    pub(crate) const END_OF_BLOCK_CODE: usize = 256;
    pub(crate) const NUMBER_OF_CODE_LENGTH_TREE_ELEMENTS: usize = 19;

    pub fn invalid() -> Self {
        HuffmanTree {
            code_lengths_length: Default::default(),
            table: [0i16; 1 << Self::TABLE_BITS],
            left: [0i16; Self::MAX_CODE_LENGTHS * 2],
            right: [0i16; Self::MAX_CODE_LENGTHS * 2],
            code_length_array: [0u8; Self::MAX_CODE_LENGTHS],
        }
    }

    pub fn static_literal_length_tree() -> Self {
        HuffmanTree::new(&Self::get_static_literal_tree_length()).unwrap()
    }

    pub fn static_distance_tree() -> Self {
        HuffmanTree::new(&Self::get_static_distance_tree_length()).unwrap()
    }

    fn assert_code_lengths_len(len: usize) {
        debug_assert!(
            len == Self::MAX_LITERAL_TREE_ELEMENTS
                || len == Self::MAX_DIST_TREE_ELEMENTS
                || len == Self::NUMBER_OF_CODE_LENGTH_TREE_ELEMENTS,
            "we only expect three kinds of Length here"
        );
    }

    pub fn new(code_lengths: &[u8]) -> Result<HuffmanTree, InternalErr> {
        Self::assert_code_lengths_len(code_lengths.len());
        let code_lengths_length = code_lengths.len();

        // I need to find proof that left and right array will always be
        // enough. I think they are.

        let mut instance = Self {
            table: [0; 1 << Self::TABLE_BITS],
            left: [0; Self::MAX_CODE_LENGTHS * 2],
            right: [0; Self::MAX_CODE_LENGTHS * 2],
            code_lengths_length: code_lengths_length as u16,
            code_length_array: {
                let mut buffer = [0u8; Self::MAX_CODE_LENGTHS];
                buffer[..code_lengths.len()].copy_from_slice(code_lengths);
                buffer
            },
        };

        instance.create_table()?;

        Ok(instance)
    }

    pub fn new_in_place(&mut self, code_lengths: &[u8]) -> Result<(), InternalErr> {
        Self::assert_code_lengths_len(code_lengths.len());
        self.table.fill(0);
        self.left.fill(0);
        self.right.fill(0);
        self.code_lengths_length = code_lengths.len() as u16;
        self.code_length_array[..code_lengths.len()].copy_from_slice(code_lengths);
        self.code_length_array[code_lengths.len()..].fill(0);

        self.create_table()
    }

    // Generate the array contains huffman codes lengths for static huffman tree.
    // The data is in RFC 1951.
    fn get_static_literal_tree_length() -> [u8; Self::MAX_LITERAL_TREE_ELEMENTS] {
        let mut literal_tree_length = [0u8; Self::MAX_LITERAL_TREE_ELEMENTS];

        literal_tree_length[0..][..144].fill(8);
        literal_tree_length[144..][..112].fill(9);
        literal_tree_length[256..][..24].fill(7);
        literal_tree_length[280..][..8].fill(8);
        literal_tree_length
    }

    const fn get_static_distance_tree_length() -> [u8; Self::MAX_DIST_TREE_ELEMENTS] {
        [5u8; Self::MAX_DIST_TREE_ELEMENTS]
    }

    fn bit_reverse(mut code: u32, mut length: usize) -> u32 {
        let mut new_code = 0;

        debug_assert!(length > 0 && length <= 16, "Invalid len");
        while {
            new_code |= code & 1;
            new_code <<= 1;
            code >>= 1;

            length -= 1;
            length > 0
        } {}

        new_code >> 1
    }

    fn calculate_huffman_code(&self) -> [u32; Self::MAX_LITERAL_TREE_ELEMENTS] {
        let mut bit_length_count = [0u32; 17];
        for &code_length in get!(self.code_length_array).iter() {
            bit_length_count[code_length as usize] += 1;
        }
        bit_length_count[0] = 0; // clear count for length 0

        let mut next_code = [0u32; 17];
        let mut temp_code = 0u32;

        for bits in 1..=16 {
            temp_code = (temp_code + bit_length_count[bits - 1]) << 1;
            next_code[bits] = temp_code;
        }

        let mut code = [0u32; Self::MAX_LITERAL_TREE_ELEMENTS];
        for (i, &len) in get!(self.code_length_array).iter().enumerate() {
            if len > 0 {
                code[i] = Self::bit_reverse(next_code[len as usize], len as usize);
                next_code[len as usize] += 1;
            }
        }

        code
    }

    fn create_table(&mut self) -> Result<(), InternalErr> {
        let code_array = self.calculate_huffman_code();

        let mut avail = get!(self.code_length_array).len() as i16;

        for (ch, &len) in get!(self.code_length_array).iter().enumerate() {
            if len > 0 {
                // start value (bit reversed)
                let mut start = code_array[ch] as usize;

                if len <= Self::TABLE_BITS {
                    // If a particular symbol is shorter than nine bits,
                    // then that symbol's translation is duplicated
                    // in all those entries that start with that symbol's bits.
                    // For example, if the symbol is four bits, then it's duplicated
                    // 32 times in a nine-bit table. If a symbol is nine bits long,
                    // it appears in the table once.
                    //
                    // Make sure that in the loop below, code is always
                    // less than table_size.
                    //
                    // On last iteration we store at array index:
                    //    initial_start_at + (locs-1)*increment
                    //  = initial_start_at + locs*increment - increment
                    //  = initial_start_at + (1 << tableBits) - increment
                    //  = initial_start_at + table_size - increment
                    //
                    // Therefore we must ensure:
                    //     initial_start_at + table_size - increment < table_size
                    // or: initial_start_at < increment
                    //
                    let increment = 1 << len;
                    if start >= increment {
                        return Err(InternalErr::DataError); // InvalidHuffmanData
                    }

                    // Note the bits in the table are reverted.
                    let locs = 1 << (Self::TABLE_BITS - len);
                    for _ in 0..locs {
                        get!(self.table)[start] = ch as i16;
                        start += increment;
                    }
                } else {
                    // For any code which has length longer than num_elements,
                    // build a binary tree.

                    let mut overflow_bits = len - Self::TABLE_BITS; // the nodes we need to represent the data.
                    let mut code_bit_mask = 1 << Self::TABLE_BITS; // mask to get current bit (the bits can't fit in the table)

                    // the left, right table is used to represent the
                    // the rest bits. When we got the first part (number bits.) and look at
                    // tbe table, we will need to follow the tree to find the real character.
                    // This is in place to avoid bloating the table if there are
                    // a few ones with long code.
                    let mut index = start & ((1 << Self::TABLE_BITS) - 1);
                    let mut array = &mut get!(self.table);

                    while {
                        let mut value = array[index];

                        if value == 0 {
                            // set up next pointer if this node is not used before.
                            array[index] = -avail; // use next available slot.
                            value = -avail;
                            avail += 1;
                        }

                        if value > 0 {
                            // prevent an IndexOutOfRangeException from array[index]
                            return Err(InternalErr::DataError); // InvalidHuffmanData
                        }

                        debug_assert!(
                            value < 0,
                            "create_table: Only negative numbers are used for tree pointers!"
                        );

                        if (start & code_bit_mask) == 0 {
                            // if current bit is 0, go change the left array
                            array = &mut get!(self.left);
                        } else {
                            // if current bit is 1, set value in the right array
                            array = &mut get!(self.right);
                        }
                        index = -value as usize; // go to next node

                        if index >= array.len() {
                            // prevent an IndexOutOfRangeException from array[index]
                            return Err(InternalErr::DataError); // InvalidHuffmanData
                        }

                        code_bit_mask <<= 1;
                        overflow_bits -= 1;

                        overflow_bits != 0
                    } {}

                    array[index] = ch as i16;
                }
            }
        }

        Ok(())
    }

    pub fn get_next_symbol(&self, input: &mut InputBuffer<'_>) -> Result<u16, InternalErr> {
        assert_ne!(self.code_lengths_length, 0, "invalid table");
        // Try to load 16 bits into input buffer if possible and get the bit_buffer value.
        // If there aren't 16 bits available we will return all we have in the
        // input buffer.
        let bit_buffer = input.try_load_16bits();
        if input.available_bits() == 0 {
            // running out of input.
            return Err(InternalErr::DataNeeded);
        }

        // decode an element
        let mut symbol = self.table[bit_buffer as usize & Self::TABLE_BITS_MASK];
        if symbol < 0 {
            //  this will be the start of the binary tree
            // navigate the tree
            let mut mask = 1 << Self::TABLE_BITS;
            while {
                symbol = -symbol;
                if (bit_buffer & mask) == 0 {
                    symbol = get!(self.left)[symbol as usize];
                } else {
                    symbol = get!(self.right)[symbol as usize];
                }
                mask <<= 1;
                symbol < 0
            } {}
        }

        debug_assert!(symbol >= 0);

        let code_length = get!(self.code_length_array)[symbol as usize] as i32;

        // huffman code lengths must be at least 1 bit long
        if code_length <= 0 {
            return Err(InternalErr::DataError); // InvalidHuffmanData
        }

        //
        // If this code is longer than the # bits we had in the bit buffer (i.e.
        // we read only part of the code), we can hit the entry in the table or the tree
        // for another symbol. However the length of another symbol will not match the
        // available bits count.
        if code_length > input.available_bits() {
            // We already tried to load 16 bits and maximum length is 15,
            // so this means we are running out of input.
            return Err(InternalErr::DataNeeded);
        }

        input.skip_bits(code_length);
        Ok(symbol as u16)
    }
}
