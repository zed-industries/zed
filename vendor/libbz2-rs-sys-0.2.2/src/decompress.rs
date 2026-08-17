#![forbid(unsafe_code)]

use core::ffi::c_int;

use crate::allocator::Allocator;
use crate::bzlib::{
    index_into_f, BzStream, DSlice, DState, DecompressMode, ReturnCode, SaveArea, BZ_MAX_SELECTORS,
    BZ_RAND_UPD_MASK, BZ_RUNA, BZ_RUNB,
};
use crate::{debug_log, huffman};

/*-- Constants for the fast MTF decoder. --*/

const MTFA_SIZE: u16 = 4096;
const MTFL_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub(crate) enum State {
    BZ_X_IDLE = 1,
    BZ_X_OUTPUT = 2,
    BZ_X_MAGIC_1 = 10,
    BZ_X_MAGIC_2 = 11,
    BZ_X_MAGIC_3 = 12,
    BZ_X_MAGIC_4 = 13,
    BZ_X_BLKHDR_1 = 14,
    BZ_X_BLKHDR_2 = 15,
    BZ_X_BLKHDR_3 = 16,
    BZ_X_BLKHDR_4 = 17,
    BZ_X_BLKHDR_5 = 18,
    BZ_X_BLKHDR_6 = 19,
    BZ_X_BCRC_1 = 20,
    BZ_X_BCRC_2 = 21,
    BZ_X_BCRC_3 = 22,
    BZ_X_BCRC_4 = 23,
    BZ_X_RANDBIT = 24,
    BZ_X_ORIGPTR_1 = 25,
    BZ_X_ORIGPTR_2 = 26,
    BZ_X_ORIGPTR_3 = 27,
    BZ_X_MAPPING_1 = 28,
    BZ_X_MAPPING_2 = 29,
    BZ_X_SELECTOR_1 = 30,
    BZ_X_SELECTOR_2 = 31,
    BZ_X_SELECTOR_3 = 32,
    BZ_X_CODING_1 = 33,
    BZ_X_CODING_2 = 34,
    BZ_X_CODING_3 = 35,
    BZ_X_MTF_1 = 36,
    BZ_X_MTF_2 = 37,
    BZ_X_MTF_3 = 38,
    BZ_X_MTF_4 = 39,
    BZ_X_MTF_5 = 40,
    BZ_X_MTF_6 = 41,
    BZ_X_ENDHDR_2 = 42,
    BZ_X_ENDHDR_3 = 43,
    BZ_X_ENDHDR_4 = 44,
    BZ_X_ENDHDR_5 = 45,
    BZ_X_ENDHDR_6 = 46,
    BZ_X_CCRC_1 = 47,
    BZ_X_CCRC_2 = 48,
    BZ_X_CCRC_3 = 49,
    BZ_X_CCRC_4 = 50,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq)]
enum Block {
    BZ_X_MAGIC_2,
    BZ_X_MAGIC_3,
    BZ_X_MAGIC_4,
    BZ_X_BLKHDR_1,
    BZ_X_BLKHDR_2,
    BZ_X_BLKHDR_3,
    BZ_X_BLKHDR_4,
    BZ_X_BLKHDR_5,
    BZ_X_BLKHDR_6,
    BZ_X_BCRC_1,
    BZ_X_BCRC_2,
    BZ_X_BCRC_3,
    BZ_X_BCRC_4,
    BZ_X_RANDBIT,
    BZ_X_ORIGPTR_1,
    BZ_X_ORIGPTR_2,
    BZ_X_ORIGPTR_3,
    BZ_X_MAPPING_1,
    BZ_X_MAPPING_2,
    BZ_X_SELECTOR_1,
    BZ_X_SELECTOR_2,
    BZ_X_SELECTOR_3,
    BZ_X_CODING_1,
    BZ_X_CODING_2,
    BZ_X_CODING_3,
    BZ_X_MTF_1,
    BZ_X_MTF_2,
    BZ_X_MTF_3,
    BZ_X_MTF_4,
    BZ_X_MTF_5,
    BZ_X_MTF_6,
    BZ_X_ENDHDR_2,
    BZ_X_ENDHDR_3,
    BZ_X_ENDHDR_4,
    BZ_X_ENDHDR_5,
    BZ_X_ENDHDR_6,
    BZ_X_CCRC_1,
    BZ_X_CCRC_2,
    BZ_X_CCRC_3,
    BZ_X_CCRC_4,
    Block1,
    Block11,
    Block18,
    Block24,
    Block25,
    Block26,
    Block28,
    Block35,
    Block39,
    Block40,
    Block41,
    Block43,
    Block45,
    Block46,
    Block51,
    Block52,
    Block56,
    Block58,
}
use Block::*;

pub(crate) fn decompress(
    strm: &mut BzStream<DState>,
    s: &mut DState,
    allocator: &Allocator,
) -> ReturnCode {
    let mut current_block: Block;
    let mut uc: u8;

    let old_avail_in = strm.avail_in;

    if let State::BZ_X_MAGIC_1 = s.state {
        /*zero out the save area*/
        s.save = SaveArea::default();
    }

    /*restore from the save area*/
    let SaveArea {
        mut i,
        mut j,
        mut t,
        mut alphaSize,
        mut nGroups,
        mut nSelectors,
        mut EOB,
        mut groupNo,
        mut groupPos,
        mut nextSym,
        mut nblockMAX100k,
        mut nblock,
        mut es,
        mut logN,
        mut curr,
        mut zn,
        mut zvec,
        mut zj,
        mut gSel,
        mut gMinlen,
    } = s.save;

    let ret_val: ReturnCode = 'save_state_and_return: {
        macro_rules! GET_BYTE {
            ($strm:expr, $s:expr) => {
                (GET_BITS!($strm, $s, 8) & 0xFF) as u8
            };
        }

        macro_rules! GET_BIT {
            ($strm:expr, $s:expr) => {
                GET_BITS!($strm, $s, 1) != 0
            };
        }

        macro_rules! GET_BITS {
            ($strm:expr, $s:expr, $nnn:expr) => {
                loop {
                    if $s.bsLive >= $nnn {
                        let v: u64 = ($s.bsBuff >> ($s.bsLive - $nnn)) & ((1 << $nnn) - 1);
                        $s.bsLive -= $nnn;
                        break v;
                    }

                    // try and read up to 8 bytes, but only if there is no risk of reading past the
                    // end of the file. This is important in a multistream scenario (where 2 bzip2
                    // files are stored back-to-back)
                    //
                    // Before `State::BZ_X_ENDHDR_2` is reached, at least 9 more bytes are expected
                    // (in a valid file), so reading 8 bytes will not cross the boundary between two files.
                    if $s.state < State::BZ_X_ENDHDR_2 {
                        if let Some((bit_buffer, bits_used)) = strm.pull_u64($s.bsBuff, $s.bsLive) {
                            $s.bsBuff = bit_buffer;
                            $s.bsLive = bits_used;
                            continue;
                        }
                    }

                    if let Some((bit_buffer, bits_used)) = strm.pull_u8($s.bsBuff, $s.bsLive) {
                        $s.bsBuff = bit_buffer;
                        $s.bsLive = bits_used;
                    } else {
                        break 'save_state_and_return ReturnCode::BZ_OK;
                    }
                }
            };
        }

        macro_rules! update_group_pos {
            ($s:expr) => {
                if groupPos == 0 {
                    groupNo += 1;
                    gSel = match $s.selector[..usize::from(nSelectors)].get(groupNo as usize) {
                        Some(&gSel) => gSel,
                        None => error!(BZ_DATA_ERROR),
                    };
                    gMinlen = $s.minLens[usize::from(gSel)];
                    groupPos = 50;
                }
                groupPos -= 1;
            };
        }

        macro_rules! error {
            ($code:ident) => {{
                break 'save_state_and_return ReturnCode::$code;
            }};
        }

        match s.state {
            State::BZ_X_MAGIC_1 => {
                s.state = State::BZ_X_MAGIC_1;

                uc = GET_BYTE!(strm, s);

                if uc != b'B' {
                    error!(BZ_DATA_ERROR_MAGIC);
                }

                current_block = BZ_X_MAGIC_2;
            }
            State::BZ_X_MAGIC_2 => current_block = BZ_X_MAGIC_2,
            State::BZ_X_MAGIC_3 => current_block = BZ_X_MAGIC_3,
            State::BZ_X_MAGIC_4 => current_block = BZ_X_MAGIC_4,
            State::BZ_X_BLKHDR_1 => current_block = BZ_X_BLKHDR_1,
            State::BZ_X_BLKHDR_2 => current_block = BZ_X_BLKHDR_2,
            State::BZ_X_BLKHDR_3 => current_block = BZ_X_BLKHDR_3,
            State::BZ_X_BLKHDR_4 => current_block = BZ_X_BLKHDR_4,
            State::BZ_X_BLKHDR_5 => current_block = BZ_X_BLKHDR_5,
            State::BZ_X_BLKHDR_6 => current_block = BZ_X_BLKHDR_6,
            State::BZ_X_BCRC_1 => current_block = BZ_X_BCRC_1,
            State::BZ_X_BCRC_2 => current_block = BZ_X_BCRC_2,
            State::BZ_X_BCRC_3 => current_block = BZ_X_BCRC_3,
            State::BZ_X_BCRC_4 => current_block = BZ_X_BCRC_4,
            State::BZ_X_RANDBIT => current_block = BZ_X_RANDBIT,
            State::BZ_X_ORIGPTR_1 => current_block = BZ_X_ORIGPTR_1,
            State::BZ_X_ORIGPTR_2 => current_block = BZ_X_ORIGPTR_2,
            State::BZ_X_ORIGPTR_3 => current_block = BZ_X_ORIGPTR_3,
            State::BZ_X_MAPPING_1 => current_block = BZ_X_MAPPING_1,
            State::BZ_X_MAPPING_2 => current_block = BZ_X_MAPPING_2,
            State::BZ_X_SELECTOR_1 => current_block = BZ_X_SELECTOR_1,
            State::BZ_X_SELECTOR_2 => current_block = BZ_X_SELECTOR_2,
            State::BZ_X_SELECTOR_3 => current_block = BZ_X_SELECTOR_3,
            State::BZ_X_CODING_1 => current_block = BZ_X_CODING_1,
            State::BZ_X_CODING_2 => current_block = BZ_X_CODING_2,
            State::BZ_X_CODING_3 => current_block = BZ_X_CODING_3,
            State::BZ_X_MTF_1 => current_block = BZ_X_MTF_1,
            State::BZ_X_MTF_2 => current_block = BZ_X_MTF_2,
            State::BZ_X_MTF_3 => current_block = BZ_X_MTF_3,
            State::BZ_X_MTF_4 => current_block = BZ_X_MTF_4,
            State::BZ_X_MTF_5 => current_block = BZ_X_MTF_5,
            State::BZ_X_MTF_6 => current_block = BZ_X_MTF_6,
            State::BZ_X_ENDHDR_2 => current_block = BZ_X_ENDHDR_2,
            State::BZ_X_ENDHDR_3 => current_block = BZ_X_ENDHDR_3,
            State::BZ_X_ENDHDR_4 => current_block = BZ_X_ENDHDR_4,
            State::BZ_X_ENDHDR_5 => current_block = BZ_X_ENDHDR_5,
            State::BZ_X_ENDHDR_6 => current_block = BZ_X_ENDHDR_6,
            State::BZ_X_CCRC_1 => current_block = BZ_X_CCRC_1,
            State::BZ_X_CCRC_2 => current_block = BZ_X_CCRC_2,
            State::BZ_X_CCRC_3 => current_block = BZ_X_CCRC_3,
            State::BZ_X_CCRC_4 => current_block = BZ_X_CCRC_4,
            State::BZ_X_IDLE | State::BZ_X_OUTPUT => unreachable!(),
        }
        if current_block == BZ_X_MAGIC_2 {
            s.state = State::BZ_X_MAGIC_2;

            uc = GET_BYTE!(strm, s);

            if uc != b'Z' {
                error!(BZ_DATA_ERROR_MAGIC);
            }

            current_block = BZ_X_MAGIC_3;
        }
        if current_block == BZ_X_MAGIC_3 {
            s.state = State::BZ_X_MAGIC_3;

            uc = GET_BYTE!(strm, s);

            if uc != b'h' {
                error!(BZ_DATA_ERROR_MAGIC);
            }

            current_block = BZ_X_MAGIC_4;
        }
        if current_block == BZ_X_MAGIC_4 {
            s.state = State::BZ_X_MAGIC_4;

            s.blockSize100k = GET_BYTE!(strm, s);

            if !(b'1'..=b'9').contains(&s.blockSize100k) {
                error!(BZ_DATA_ERROR_MAGIC);
            }

            s.blockSize100k -= b'0';

            match s.smallDecompress {
                DecompressMode::Small => {
                    // SAFETY: we assume allocation is safe
                    let ll16_len = usize::from(s.blockSize100k) * 100000;
                    let Some(ll16) = DSlice::alloc(allocator, ll16_len) else {
                        error!(BZ_MEM_ERROR);
                    };

                    // SAFETY: we assume allocation is safe
                    let ll4_len = (1 + usize::from(s.blockSize100k) * 100000) >> 1;
                    let Some(ll4) = DSlice::alloc(allocator, ll4_len) else {
                        error!(BZ_MEM_ERROR);
                    };

                    s.ll16 = ll16;
                    s.ll4 = ll4;
                }
                DecompressMode::Fast => {
                    // SAFETY: we assume allocation is safe
                    let tt_len = usize::from(s.blockSize100k) * 100000;
                    let Some(tt) = DSlice::alloc(allocator, tt_len) else {
                        error!(BZ_MEM_ERROR);
                    };

                    s.tt = tt;
                }
            }

            current_block = BZ_X_BLKHDR_1;
        }
        if current_block == BZ_X_BLKHDR_1 {
            s.state = State::BZ_X_BLKHDR_1;

            uc = GET_BYTE!(strm, s);

            match uc {
                0x17 => current_block = BZ_X_ENDHDR_2,
                0x31 => current_block = BZ_X_BLKHDR_2,
                _ => error!(BZ_DATA_ERROR),
            };
        }
        match current_block {
            BZ_X_ENDHDR_2 => {
                s.state = State::BZ_X_ENDHDR_2;

                uc = GET_BYTE!(strm, s);

                if uc != 0x72 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_ENDHDR_3;
            }
            BZ_X_BLKHDR_2 => {
                s.state = State::BZ_X_BLKHDR_2;

                uc = GET_BYTE!(strm, s);

                if uc != 0x41 {
                    error!(BZ_DATA_ERROR);
                }
                current_block = BZ_X_BLKHDR_3;
            }
            _ => {}
        }
        match current_block {
            BZ_X_ENDHDR_3 => {
                s.state = State::BZ_X_ENDHDR_3;

                uc = GET_BYTE!(strm, s);

                if uc != 0x45 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_ENDHDR_4;
            }
            BZ_X_BLKHDR_3 => {
                s.state = State::BZ_X_BLKHDR_3;

                uc = GET_BYTE!(strm, s);

                if uc != 0x59 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_BLKHDR_4;
            }
            _ => {}
        }
        match current_block {
            BZ_X_ENDHDR_4 => {
                s.state = State::BZ_X_ENDHDR_4;

                uc = GET_BYTE!(strm, s);

                if uc != 0x38 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_ENDHDR_5;
            }
            BZ_X_BLKHDR_4 => {
                s.state = State::BZ_X_BLKHDR_4;

                uc = GET_BYTE!(strm, s);

                if uc != 0x26 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_BLKHDR_5;
            }
            _ => {}
        }
        match current_block {
            BZ_X_ENDHDR_5 => {
                s.state = State::BZ_X_ENDHDR_5;

                uc = GET_BYTE!(strm, s);

                if uc != 0x50 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_ENDHDR_6;
            }
            BZ_X_BLKHDR_5 => {
                s.state = State::BZ_X_BLKHDR_5;

                uc = GET_BYTE!(strm, s);

                if uc != 0x53 {
                    error!(BZ_DATA_ERROR);
                }

                current_block = BZ_X_BLKHDR_6;
            }
            _ => {}
        }
        match current_block {
            BZ_X_ENDHDR_6 => {
                s.state = State::BZ_X_ENDHDR_6;

                uc = GET_BYTE!(strm, s);

                if uc != 0x90 {
                    error!(BZ_DATA_ERROR);
                }

                s.storedCombinedCRC = 0_u32;
                current_block = BZ_X_CCRC_1;
            }
            BZ_X_BLKHDR_6 => {
                s.state = State::BZ_X_BLKHDR_6;

                uc = GET_BYTE!(strm, s);

                if uc != 0x59 {
                    error!(BZ_DATA_ERROR);
                }

                s.currBlockNo += 1;
                if s.verbosity >= 2 {
                    debug_log!("\n    [{}: huff+mtf ", s.currBlockNo);
                }
                s.storedBlockCRC = 0_u32;
                current_block = BZ_X_BCRC_1;
            }
            _ => {}
        }
        match current_block {
            BZ_X_CCRC_1 => {
                s.state = State::BZ_X_CCRC_1;

                uc = GET_BYTE!(strm, s);

                s.storedCombinedCRC = (s.storedCombinedCRC << 8) | uc as u32;
                current_block = BZ_X_CCRC_2;
            }
            BZ_X_BCRC_1 => {
                s.state = State::BZ_X_BCRC_1;

                uc = GET_BYTE!(strm, s);

                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                current_block = BZ_X_BCRC_2;
            }
            _ => {}
        }
        match current_block {
            BZ_X_CCRC_2 => {
                s.state = State::BZ_X_CCRC_2;

                uc = GET_BYTE!(strm, s);

                s.storedCombinedCRC = (s.storedCombinedCRC << 8) | uc as u32;
                current_block = BZ_X_CCRC_3;
            }
            BZ_X_BCRC_2 => {
                s.state = State::BZ_X_BCRC_2;

                uc = GET_BYTE!(strm, s);

                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                current_block = BZ_X_BCRC_3;
            }
            _ => {}
        }
        match current_block {
            BZ_X_CCRC_3 => {
                s.state = State::BZ_X_CCRC_3;

                uc = GET_BYTE!(strm, s);

                s.storedCombinedCRC = (s.storedCombinedCRC << 8) | uc as u32;
                current_block = BZ_X_CCRC_4;
            }
            BZ_X_BCRC_3 => {
                s.state = State::BZ_X_BCRC_3;

                uc = GET_BYTE!(strm, s);

                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                current_block = BZ_X_BCRC_4;
            }
            _ => {}
        }
        match current_block {
            BZ_X_BCRC_4 => {
                s.state = State::BZ_X_BCRC_4;

                uc = GET_BYTE!(strm, s);

                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                current_block = BZ_X_RANDBIT;
            }
            BZ_X_CCRC_4 => {
                s.state = State::BZ_X_CCRC_4;

                uc = GET_BYTE!(strm, s);

                s.storedCombinedCRC = (s.storedCombinedCRC << 8) | uc as u32;
                s.state = State::BZ_X_IDLE;
                error!(BZ_STREAM_END);
            }
            _ => {}
        }
        if current_block == BZ_X_RANDBIT {
            s.state = State::BZ_X_RANDBIT;

            s.blockRandomised = GET_BITS!(strm, s, 1) != 0;

            s.origPtr = 0;
            current_block = BZ_X_ORIGPTR_1;
        }
        if current_block == BZ_X_ORIGPTR_1 {
            s.state = State::BZ_X_ORIGPTR_1;

            uc = GET_BYTE!(strm, s);

            s.origPtr = (s.origPtr << 8) | i32::from(uc);
            current_block = BZ_X_ORIGPTR_2;
        }
        if current_block == BZ_X_ORIGPTR_2 {
            s.state = State::BZ_X_ORIGPTR_2;

            uc = GET_BYTE!(strm, s);

            s.origPtr = (s.origPtr << 8) | i32::from(uc);
            current_block = BZ_X_ORIGPTR_3;
        }
        if current_block == BZ_X_ORIGPTR_3 {
            s.state = State::BZ_X_ORIGPTR_3;

            uc = GET_BYTE!(strm, s);

            s.origPtr = (s.origPtr << 8) | i32::from(uc);
            if !(0..=10 + 100000 * i32::from(s.blockSize100k)).contains(&s.origPtr) {
                error!(BZ_DATA_ERROR);
            }

            i = 0;
            current_block = Block43;
        }

        // mutable because they need to be reborrowed
        let tt = s.tt.as_mut_slice();
        let ll16 = s.ll16.as_mut_slice();
        let ll4 = s.ll4.as_mut_slice();

        'state_machine: loop {
            match current_block {
                BZ_X_MAPPING_1 => {
                    s.state = State::BZ_X_MAPPING_1;

                    uc = GET_BIT!(strm, s) as u8;

                    s.inUse16[i as usize] = uc == 1;
                    i += 1;
                    current_block = Block43;
                    continue;
                }
                Block43 => {
                    if i < 16 {
                        current_block = BZ_X_MAPPING_1;
                        continue;
                    }
                    s.inUse.fill(false);
                    i = 0;
                    current_block = Block18;
                }
                BZ_X_MAPPING_2 => {
                    s.state = State::BZ_X_MAPPING_2;

                    uc = GET_BIT!(strm, s) as u8;

                    if uc == 1 {
                        s.inUse[(i * 16 + j) as usize] = true;
                    }
                    j += 1;
                    current_block = Block28;
                }
                BZ_X_SELECTOR_1 => {
                    s.state = State::BZ_X_SELECTOR_1;

                    nGroups = GET_BITS!(strm, s, 3) as u8;

                    if (2..=6).contains(&nGroups) {
                        current_block = BZ_X_SELECTOR_2;
                        continue;
                    }
                    error!(BZ_DATA_ERROR);
                }
                BZ_X_SELECTOR_2 => {
                    s.state = State::BZ_X_SELECTOR_2;

                    nSelectors = GET_BITS!(strm, s, 15) as u16;

                    if nSelectors < 1 {
                        error!(BZ_DATA_ERROR);
                    } else {
                        i = 0;
                    }
                    current_block = Block39;
                }
                BZ_X_SELECTOR_3 => {
                    s.state = State::BZ_X_SELECTOR_3;

                    uc = GET_BIT!(strm, s) as u8;

                    if uc == 0 {
                        current_block = Block1;
                    } else {
                        j += 1;
                        if j >= i32::from(nGroups) {
                            error!(BZ_DATA_ERROR);
                        } else {
                            current_block = Block25;
                        }
                    }
                }
                BZ_X_CODING_1 => {
                    s.state = State::BZ_X_CODING_1;

                    curr = GET_BITS!(strm, s, 5) as u8;

                    i = 0;
                    current_block = Block26;
                }
                BZ_X_CODING_2 => {
                    s.state = State::BZ_X_CODING_2;

                    uc = GET_BIT!(strm, s) as u8;

                    if uc != 0 {
                        current_block = BZ_X_CODING_3;
                        continue;
                    }
                    current_block = Block51;
                }
                BZ_X_CODING_3 => {
                    s.state = State::BZ_X_CODING_3;

                    uc = GET_BIT!(strm, s) as u8;

                    match uc {
                        0 => curr += 1,
                        _ => curr -= 1,
                    }

                    current_block = Block45;
                }
                BZ_X_MTF_1 => {
                    s.state = State::BZ_X_MTF_1;

                    zvec = GET_BITS!(strm, s, zn as i32) as i32;

                    current_block = Block56;
                }
                BZ_X_MTF_2 => {
                    s.state = State::BZ_X_MTF_2;

                    zj = GET_BIT!(strm, s);

                    zvec = (zvec << 1) | zj as i32;
                    current_block = Block56;
                }
                BZ_X_MTF_3 => {
                    s.state = State::BZ_X_MTF_3;

                    zvec = GET_BITS!(strm, s, zn as i32) as i32;

                    current_block = Block52;
                }
                BZ_X_MTF_4 => {
                    s.state = State::BZ_X_MTF_4;

                    zj = GET_BIT!(strm, s);

                    zvec = (zvec << 1) | zj as i32;
                    current_block = Block52;
                }
                BZ_X_MTF_5 => {
                    s.state = State::BZ_X_MTF_5;

                    zvec = GET_BITS!(strm, s, zn as i32) as i32;

                    current_block = Block24;
                }
                _ => {
                    s.state = State::BZ_X_MTF_6;

                    zj = GET_BIT!(strm, s);

                    zvec = (zvec << 1) | zj as i32;
                    current_block = Block24;
                }
            }

            macro_rules! get_next_sym {
                ($next_block:ident) => {
                    if zn > 20 {
                        // zn is higher than the longest code, that's invalid input
                        error!(BZ_DATA_ERROR);
                    } else if zvec <= s.limit[usize::from(gSel)][zn as usize] {
                        let index = zvec - s.base[usize::from(gSel)][zn as usize];
                        match s.perm[usize::from(gSel)].get(index as usize) {
                            Some(&nextSym) => nextSym,
                            None => error!(BZ_DATA_ERROR),
                        }
                    } else {
                        zn += 1;
                        current_block = $next_block;
                        continue 'state_machine;
                    }
                };
            }

            match current_block {
                Block24 => {
                    nextSym = get_next_sym!(BZ_X_MTF_6);
                    current_block = Block40;
                }
                Block52 => {
                    nextSym = get_next_sym!(BZ_X_MTF_4);

                    if nextSym == BZ_RUNA || nextSym == BZ_RUNB {
                        current_block = Block46;
                    } else {
                        let uc = s.seqToUnseq[usize::from(s.mtfa[usize::from(s.mtfbase[0])])];
                        s.unzftab[usize::from(uc)] += es;
                        match s.smallDecompress {
                            DecompressMode::Small => {
                                match ll16.get_mut(nblock as usize..(nblock + es) as usize) {
                                    Some(slice) => slice.fill(u16::from(uc)),
                                    None => error!(BZ_DATA_ERROR),
                                };
                                nblock += es;
                            }
                            DecompressMode::Fast => {
                                match tt.get_mut(nblock as usize..(nblock + es) as usize) {
                                    Some(slice) => slice.fill(u32::from(uc)),
                                    None => error!(BZ_DATA_ERROR),
                                };
                                nblock += es;
                            }
                        }
                        current_block = Block40;
                    }
                }
                Block56 => {
                    nextSym = get_next_sym!(BZ_X_MTF_2);
                    current_block = Block40;
                }
                _ => {}
            }
            if current_block == Block40 {
                if nextSym == EOB {
                    current_block = Block41;
                } else if nextSym == BZ_RUNA || nextSym == BZ_RUNB {
                    es = 0;
                    logN = 0;
                    current_block = Block46;
                } else if nblock >= 100000 * u32::from(nblockMAX100k) {
                    error!(BZ_DATA_ERROR);
                } else {
                    let uc = usize::from(initialize_mtfa(&mut s.mtfa, &mut s.mtfbase, nextSym));
                    let index = s.seqToUnseq[uc];
                    s.unzftab[usize::from(index)] += 1;
                    match s.smallDecompress {
                        DecompressMode::Small => ll16[nblock as usize] = u16::from(index),
                        DecompressMode::Fast => tt[nblock as usize] = u32::from(index),
                    }
                    nblock += 1;
                    update_group_pos!(s);
                    zn = gMinlen;
                    current_block = BZ_X_MTF_5;
                    continue;
                }
                match current_block {
                    Block46 => {}
                    _ => {
                        if s.origPtr < 0 || s.origPtr >= nblock as i32 {
                            error!(BZ_DATA_ERROR);
                        } else {
                            if s.unzftab.iter().any(|e| !(0..=nblock).contains(e)) {
                                error!(BZ_DATA_ERROR);
                            }
                            s.cftab[0] = 0;
                            s.cftab[1..].copy_from_slice(&s.unzftab);
                            for i in 1..s.cftab.len() {
                                s.cftab[i] += s.cftab[i - 1];
                            }
                            if s.cftab.iter().any(|e| !(0..=nblock).contains(e)) {
                                error!(BZ_DATA_ERROR);
                            }
                            // FIXME: use https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted
                            // when available in our MSRV (requires >= 1.82.0)
                            if s.cftab.windows(2).any(|w| w[0] > w[1]) {
                                error!(BZ_DATA_ERROR);
                            }
                            s.state_out_len = 0;
                            s.state_out_ch = 0;
                            s.calculatedBlockCRC = u32::MAX;
                            s.state = State::BZ_X_OUTPUT;
                            if s.verbosity >= 2 {
                                debug_log!("rt+rld");
                            }
                            match s.smallDecompress {
                                DecompressMode::Small => {
                                    // Make a copy of cftab, used in generation of T
                                    s.cftabCopy = s.cftab;

                                    // compute the T vector
                                    for i in 0..nblock as usize {
                                        let uc = usize::from(ll16[i]);
                                        ll16[i] = (s.cftabCopy[uc] & 0xffff) as u16;

                                        // set the lower or higher nibble depending on i
                                        let (mask, shift) = match i & 0x1 {
                                            0 => (0xF0, 0),
                                            _ => (0x0F, 4),
                                        };
                                        ll4[i / 2] &= mask;
                                        ll4[i / 2] |= ((s.cftabCopy[uc] >> 16) << shift) as u8;

                                        s.cftabCopy[uc] += 1;
                                    }

                                    // Compute T^(-1) by pointer reversal on T
                                    i = s.origPtr;
                                    j = (ll16[i as usize] as u32
                                        | (((ll4[(i >> 1) as usize] as u32 >> ((i << 2) & 0b100))
                                            & 0xf)
                                            << 16)) as i32;
                                    loop {
                                        let tmp_0: i32 = (ll16[j as usize] as u32
                                            | (((ll4[(j >> 1) as usize] as u32
                                                >> ((j << 2) & 0b100))
                                                & 0xf)
                                                << 16))
                                            as i32;
                                        ll16[j as usize] = (i & 0xffff) as u16;
                                        if j & 0x1 == 0 {
                                            ll4[(j >> 1) as usize] = (ll4[(j >> 1) as usize]
                                                as c_int
                                                & 0xf0
                                                | (i >> 16))
                                                as u8;
                                        } else {
                                            ll4[(j >> 1) as usize] =
                                                (ll4[(j >> 1) as usize] as c_int & 0xf
                                                    | ((i >> 16) << 4))
                                                    as u8
                                        }
                                        i = j;
                                        j = tmp_0;
                                        if i == s.origPtr {
                                            break;
                                        }
                                    }

                                    s.tPos = s.origPtr as u32;
                                    s.nblock_used = 0;

                                    s.k0 = index_into_f(s.tPos, &s.cftab);
                                    s.tPos = match ll16.get(s.tPos as usize) {
                                        None => error!(BZ_DATA_ERROR),
                                        Some(&low_bits) => {
                                            let high_bits = (ll4[(s.tPos >> 1) as usize]
                                                >> ((s.tPos << 2) & 0b100))
                                                & 0xf;
                                            u32::from(low_bits) | (u32::from(high_bits) << 16)
                                        }
                                    };
                                    s.nblock_used += 1;

                                    if s.blockRandomised {
                                        s.rNToGo = 0;
                                        s.rTPos = 0;
                                        BZ_RAND_UPD_MASK!(s);
                                        s.k0 ^= u8::from(s.rNToGo == 1)
                                    }
                                }
                                DecompressMode::Fast => {
                                    for i in 0..nblock as usize {
                                        let uc = (tt[i] & 0xff) as usize;
                                        tt[s.cftab[uc] as usize] |= (i << 8) as u32;
                                        s.cftab[uc] += 1;
                                    }
                                    s.tPos = tt[s.origPtr as usize] >> 8;
                                    s.nblock_used = 0;

                                    s.tPos = match tt.get(s.tPos as usize) {
                                        Some(&tPos) => tPos,
                                        None => error!(BZ_DATA_ERROR),
                                    };
                                    s.k0 = (s.tPos & 0xff) as u8;
                                    s.tPos >>= 8;
                                    s.nblock_used += 1;

                                    if s.blockRandomised {
                                        s.rNToGo = 0;
                                        s.rTPos = 0;
                                        BZ_RAND_UPD_MASK!(s);
                                        s.k0 ^= u8::from(s.rNToGo == 1)
                                    }
                                }
                            }

                            break 'save_state_and_return ReturnCode::BZ_OK;
                        }
                    }
                }
            }
            if current_block == Block46 {
                // Check that N doesn't get too big, so that es doesn't
                // go negative.  The maximum value that can be
                // RUNA/RUNB encoded is equal to the block size (post
                // the initial RLE), viz, 900k, so bounding N at 2
                // million should guard against overflow without
                // rejecting any legitimate inputs.
                const LOG_2MB: u8 = 21; // 2 * 1024 * 1024

                if logN >= LOG_2MB {
                    error!(BZ_DATA_ERROR);
                } else {
                    let mul = match nextSym {
                        BZ_RUNA => 1,
                        BZ_RUNB => 2,
                        _ => 0,
                    };
                    es += mul * (1 << logN);
                    logN += 1;
                    update_group_pos!(s);
                    zn = gMinlen;
                    current_block = BZ_X_MTF_3;
                    continue;
                }
            }
            loop {
                match current_block {
                    Block28 => {
                        if j < 16 {
                            current_block = BZ_X_MAPPING_2;
                            continue 'state_machine;
                        }
                    }
                    Block39 => {
                        if i < i32::from(nSelectors) {
                            j = 0;
                            current_block = Block25;
                            continue;
                        } else {
                            // make sure that the constant fits in a u16
                            nSelectors = Ord::min(nSelectors, BZ_MAX_SELECTORS);

                            let mut pos: [u8; 6] = [0, 1, 2, 3, 4, 5];
                            for i in 0..usize::from(nSelectors) {
                                rotate_right_1(&mut pos[..=usize::from(s.selectorMtf[i])]);
                                s.selector[i] = pos[0];
                            }

                            // try to read the coding tables in one go if there is sufficient input
                            let bits_needed =
                                usize::from(nGroups) * (5 + (usize::from(alphaSize) * 2 * 20));
                            let bytes_needed = bits_needed.div_ceil(8);

                            if strm.avail_in as usize >= bytes_needed {
                                for t in 0..usize::from(nGroups) {
                                    let mut curr = GET_BITS!(strm, s, 5);
                                    for i in 0..usize::from(alphaSize) {
                                        loop {
                                            if !(1..=20).contains(&curr) {
                                                error!(BZ_DATA_ERROR);
                                            }
                                            if !GET_BIT!(strm, s) {
                                                break;
                                            };
                                            match GET_BIT!(strm, s) {
                                                false => curr += 1,
                                                true => curr -= 1,
                                            }
                                        }

                                        s.len[t][i] = curr as u8;
                                    }
                                }

                                t = nGroups;
                                current_block = Block35;
                                break;
                            } else {
                                t = 0;
                                current_block = Block35;
                                break;
                            }
                        }
                    }
                    Block18 => {
                        if let Some(&in_use) = s.inUse16.get(i as usize) {
                            if in_use {
                                j = 0;
                                current_block = Block28;
                                continue;
                            }
                        } else {
                            // inlined `make_maps_d`
                            s.nInUse = 0;
                            for (i, in_use) in s.inUse.iter().enumerate() {
                                if *in_use {
                                    s.seqToUnseq[usize::from(s.nInUse)] = i as u8;
                                    s.nInUse += 1;
                                }
                            }

                            if s.nInUse == 0 {
                                current_block = Block11;
                                break;
                            } else {
                                current_block = Block58;
                                break;
                            }
                        }
                    }
                    Block51 => {
                        s.len[t as usize][i as usize] = curr;
                        i += 1;
                        current_block = Block26;
                        continue;
                    }
                    Block26 => {
                        if i < i32::from(alphaSize) {
                            current_block = Block45;
                            continue;
                        }
                        t += 1;
                        current_block = Block35;
                        break;
                    }
                    Block1 => {
                        if i < i32::from(BZ_MAX_SELECTORS) {
                            s.selectorMtf[i as usize] = j as u8;
                        }
                        i += 1;
                        current_block = Block39;
                        continue;
                    }
                    Block25 => {
                        current_block = BZ_X_SELECTOR_3;
                        continue 'state_machine;
                    }
                    _ => {
                        if false {
                            current_block = Block51;
                            continue;
                        }
                        if (1..=20).contains(&curr) {
                            current_block = BZ_X_CODING_2;
                            continue 'state_machine;
                        }
                        error!(BZ_DATA_ERROR);
                    }
                }
                i += 1;
                current_block = Block18;
            }
            match current_block {
                Block58 => {
                    alphaSize = s.nInUse + 2;
                    current_block = BZ_X_SELECTOR_1;
                }
                Block11 => {
                    error!(BZ_DATA_ERROR);
                }
                _ => {
                    if t < nGroups {
                        current_block = BZ_X_CODING_1;
                        continue;
                    }

                    /*--- Create the Huffman decoding tables ---*/
                    for t in 0..usize::from(nGroups) {
                        // NOTE: s.nInUse <= 256, alphaSize <= 258
                        let len = &s.len[t][..usize::from(alphaSize)];

                        let mut minLen = 32u8;
                        let mut maxLen = 0u8;
                        for &current in len {
                            maxLen = Ord::max(maxLen, current);
                            minLen = Ord::min(minLen, current);
                        }
                        s.minLens[t] = minLen;

                        huffman::create_decode_tables(
                            &mut s.limit[t],
                            &mut s.base[t],
                            &mut s.perm[t],
                            len,
                            minLen,
                            maxLen,
                        );
                    }

                    /*--- Now the MTF values ---*/

                    EOB = s.nInUse + 1;
                    nblockMAX100k = s.blockSize100k;
                    s.unzftab.fill(0);

                    /*-- MTF init --*/
                    let mut kk: u16 = MTFA_SIZE - 1;
                    for ii in (0..256 / MTFL_SIZE).rev() {
                        for jj in (0..MTFL_SIZE).rev() {
                            s.mtfa[usize::from(kk)] = (ii * MTFL_SIZE + jj) as u8;
                            kk -= 1;
                        }
                        s.mtfbase[ii] = kk + 1;
                    }
                    /*-- end MTF init --*/

                    nblock = 0;
                    groupNo = -1;
                    groupPos = 0;
                    update_group_pos!(s);

                    zn = gMinlen;
                    current_block = BZ_X_MTF_1;
                }
            }
        }
    };

    s.save = SaveArea {
        i,
        j,
        t,
        alphaSize,
        nGroups,
        nSelectors,
        EOB,
        groupNo,
        groupPos,
        nextSym,
        nblockMAX100k,
        nblock,
        es,
        logN,
        curr,
        zn,
        zvec,
        zj,
        gSel,
        gMinlen,
    };

    // update total_in with how many bytes were read during this call
    let bytes_read = old_avail_in - strm.avail_in;
    let old_total_in_lo32 = strm.total_in_lo32;
    strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(bytes_read);
    strm.total_in_hi32 += (strm.total_in_lo32 < old_total_in_lo32) as u32;

    ret_val
}

fn initialize_mtfa(mtfa: &mut [u8; 4096], mtfbase: &mut [u16; 16], nextSym: u16) -> u8 {
    let nn = usize::from(nextSym - 1);

    if nn < MTFL_SIZE {
        // avoid general case expense
        let pp = usize::from(mtfbase[0]);
        let uc = mtfa[pp + nn];
        rotate_right_1(&mut mtfa[pp..][..=nn]);

        uc
    } else {
        // general case
        let mut lno = nn.wrapping_div(MTFL_SIZE);
        let off = nn.wrapping_rem(MTFL_SIZE);
        let base = usize::from(mtfbase[lno]);
        let uc = mtfa[base + off];

        // shift this range one to the right
        mtfa.copy_within(base..base + off, base + 1);

        mtfbase[lno] += 1;
        while lno > 0 {
            mtfbase[lno] -= 1;
            mtfa[usize::from(mtfbase[lno])] = mtfa[usize::from(mtfbase[lno - 1] + 16 - 1)];
            lno -= 1;
        }
        mtfbase[0] -= 1;
        mtfa[usize::from(mtfbase[0])] = uc;

        if mtfbase[0] == 0 {
            let mut kk = MTFA_SIZE - 1;
            for ii in (0..256 / MTFL_SIZE).rev() {
                for jj in (0..MTFL_SIZE).rev() {
                    mtfa[usize::from(kk)] = mtfa[usize::from(mtfbase[ii]) + jj];
                    kk -= 1;
                }
                mtfbase[ii] = kk + 1;
            }
        }

        uc
    }
}

fn rotate_right_1(slice: &mut [u8]) {
    match slice {
        [] | [_] => { /* ignore */ }
        [a, b] => {
            // The 2-element case is fairly common, and because we already branch on the length,
            // the check for `len == 2` is very cheap.
            //
            // On x86_64 the `rol` instruction is used to swap the bytes with just 1 instruction.
            // See https://godbolt.org/z/385K7qs91
            core::mem::swap(a, b)
        }
        [.., last] => {
            let last = *last;
            slice.copy_within(0..slice.len() - 1, 1);
            slice[0] = last;
        }
    }
}
