#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use super::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE};

static _khmer_syllable_machine_actions: [i8; 29] = [
    0, 1, 0, 1, 1, 1, 2, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11, 2, 2, 3, 2, 2, 4, 0, 0,
];
static _khmer_syllable_machine_key_offsets: [i16; 45] = [
    0, 5, 8, 11, 15, 18, 21, 25, 28, 32, 35, 40, 45, 48, 51, 55, 58, 61, 65, 68, 72, 75, 90, 100,
    103, 113, 122, 123, 129, 134, 141, 149, 158, 168, 171, 181, 190, 191, 197, 202, 209, 217, 226,
    0, 0,
];
static _khmer_syllable_machine_trans_keys: [u8; 232] = [
    20, 25, 26, 5, 6, 26, 5, 6, 15, 1, 2, 20, 26, 5, 6, 26, 5, 6, 26, 5, 6, 20, 26, 5, 6, 26, 5, 6,
    20, 26, 5, 6, 26, 5, 6, 20, 25, 26, 5, 6, 20, 25, 26, 5, 6, 26, 5, 6, 15, 1, 2, 20, 26, 5, 6,
    26, 5, 6, 26, 5, 6, 20, 26, 5, 6, 26, 5, 6, 20, 26, 5, 6, 26, 5, 6, 4, 15, 20, 21, 22, 23, 25,
    26, 27, 1, 2, 5, 6, 10, 11, 4, 20, 21, 22, 23, 25, 26, 27, 5, 6, 15, 1, 2, 4, 20, 21, 22, 23,
    25, 26, 27, 5, 6, 4, 20, 21, 22, 23, 26, 27, 5, 6, 27, 4, 23, 26, 27, 5, 6, 4, 26, 27, 5, 6, 4,
    20, 23, 26, 27, 5, 6, 4, 20, 21, 23, 26, 27, 5, 6, 4, 20, 21, 22, 23, 26, 27, 5, 6, 4, 20, 21,
    22, 23, 25, 26, 27, 5, 6, 15, 1, 2, 4, 20, 21, 22, 23, 25, 26, 27, 5, 6, 4, 20, 21, 22, 23, 26,
    27, 5, 6, 27, 4, 23, 26, 27, 5, 6, 4, 26, 27, 5, 6, 4, 20, 23, 26, 27, 5, 6, 4, 20, 21, 23, 26,
    27, 5, 6, 4, 20, 21, 22, 23, 26, 27, 5, 6, 20, 26, 5, 6, 0, 0,
];
static _khmer_syllable_machine_single_lengths: [i8; 45] = [
    3, 1, 1, 2, 1, 1, 2, 1, 2, 1, 3, 3, 1, 1, 2, 1, 1, 2, 1, 2, 1, 9, 8, 1, 8, 7, 1, 4, 3, 5, 6, 7,
    8, 1, 8, 7, 1, 4, 3, 5, 6, 7, 2, 0, 0,
];
static _khmer_syllable_machine_range_lengths: [i8; 45] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0,
];
static _khmer_syllable_machine_index_offsets: [i16; 45] = [
    0, 5, 8, 11, 15, 18, 21, 25, 28, 32, 35, 40, 45, 48, 51, 55, 58, 61, 65, 68, 72, 75, 88, 98,
    101, 111, 120, 122, 128, 133, 140, 148, 157, 167, 170, 180, 189, 191, 197, 202, 209, 217, 226,
    0, 0,
];
static _khmer_syllable_machine_cond_targs: [i8; 275] = [
    27, 31, 25, 1, 21, 25, 1, 21, 26, 26, 21, 27, 25, 1, 21, 27, 4, 21, 28, 5, 21, 27, 29, 7, 21,
    29, 7, 21, 27, 30, 9, 21, 30, 9, 21, 27, 32, 25, 1, 21, 37, 41, 35, 12, 21, 35, 12, 21, 36, 36,
    21, 37, 35, 12, 21, 37, 15, 21, 38, 16, 21, 37, 39, 18, 21, 39, 18, 21, 37, 40, 20, 21, 40, 20,
    21, 33, 22, 37, 39, 40, 38, 41, 35, 36, 22, 42, 32, 21, 23, 27, 29, 30, 28, 32, 25, 26, 10, 21,
    24, 24, 21, 23, 27, 29, 30, 28, 31, 25, 26, 0, 21, 2, 27, 29, 30, 28, 25, 26, 3, 21, 26, 21, 2,
    28, 27, 26, 4, 21, 2, 28, 26, 5, 21, 2, 27, 28, 29, 26, 6, 21, 2, 27, 29, 28, 30, 26, 8, 21,
    23, 27, 29, 30, 28, 25, 26, 3, 21, 23, 27, 29, 30, 28, 31, 25, 26, 3, 21, 34, 34, 21, 33, 37,
    39, 40, 38, 41, 35, 36, 11, 21, 13, 37, 39, 40, 38, 35, 36, 14, 21, 36, 21, 13, 38, 37, 36, 15,
    21, 13, 38, 36, 16, 21, 13, 37, 38, 39, 36, 17, 21, 13, 37, 39, 38, 40, 36, 19, 21, 33, 37, 39,
    40, 38, 35, 36, 14, 21, 37, 35, 12, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 0, 0,
];
static _khmer_syllable_machine_cond_actions: [i8; 275] = [
    5, 5, 5, 0, 15, 5, 0, 15, 0, 0, 15, 5, 5, 0, 15, 5, 0, 15, 5, 0, 15, 5, 5, 0, 15, 5, 0, 15, 5,
    5, 0, 15, 5, 0, 15, 5, 5, 5, 0, 15, 5, 21, 21, 0, 17, 21, 0, 19, 0, 0, 17, 5, 21, 0, 17, 5, 0,
    17, 5, 0, 17, 5, 5, 0, 17, 5, 0, 17, 5, 5, 0, 17, 5, 0, 17, 0, 5, 5, 5, 5, 5, 21, 21, 0, 5, 24,
    5, 7, 0, 5, 5, 5, 5, 5, 5, 0, 0, 9, 5, 5, 9, 0, 5, 5, 5, 5, 5, 5, 0, 0, 9, 0, 5, 5, 5, 5, 5, 0,
    0, 9, 0, 9, 0, 5, 5, 0, 0, 9, 0, 5, 0, 0, 9, 0, 5, 5, 5, 0, 0, 9, 0, 5, 5, 5, 5, 0, 0, 9, 0, 5,
    5, 5, 5, 5, 0, 0, 9, 0, 5, 5, 5, 5, 5, 5, 0, 0, 9, 21, 21, 11, 0, 5, 5, 5, 5, 21, 21, 0, 0, 11,
    0, 5, 5, 5, 5, 21, 0, 0, 11, 0, 11, 0, 5, 5, 0, 0, 11, 0, 5, 0, 0, 11, 0, 5, 5, 5, 0, 0, 11, 0,
    5, 5, 5, 5, 0, 0, 11, 0, 5, 5, 5, 5, 21, 0, 0, 11, 5, 21, 0, 13, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 17, 19, 17, 17, 17, 17, 17, 17, 17, 17, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 13, 0, 0,
];
static _khmer_syllable_machine_to_state_actions: [i8; 45] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _khmer_syllable_machine_from_state_actions: [i8; 45] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _khmer_syllable_machine_eof_trans: [i16; 45] = [
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 0, 0,
];
static khmer_syllable_machine_start: i32 = 21;
static khmer_syllable_machine_first_final: i32 = 21;
static khmer_syllable_machine_error: i32 = -1;
static khmer_syllable_machine_en_main: i32 = 21;
#[derive(Clone, Copy)]
pub enum SyllableType {
    ConsonantSyllable = 0,
    BrokenCluster,
    NonKhmerCluster,
}

pub fn find_syllables_khmer(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let mut ts = 0;
    let mut te = 0;
    let mut act = 0;
    let mut p = 0;
    let pe = buffer.len;
    let eof = buffer.len;
    let mut syllable_serial = 1u8;

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts, te, &mut syllable_serial, $kind, buffer);
        }};
    }

    {
        cs = (khmer_syllable_machine_start) as i32;
        ts = 0;
        te = 0;
        act = 0;
    }

    {
        let mut _klen = 0;
        let mut _trans = 0;
        let mut _keys: i32 = 0;
        let mut _acts: i32 = 0;
        let mut _nacts = 0;
        let mut __have = 0;
        '_resume: while (p != pe || p == eof) {
            '_again: while (true) {
                _acts = (_khmer_syllable_machine_from_state_actions[(cs) as usize]) as i32;
                _nacts = (_khmer_syllable_machine_actions[(_acts) as usize]) as u32;
                _acts += 1;
                while (_nacts > 0) {
                    match (_khmer_syllable_machine_actions[(_acts) as usize]) {
                        1 => {
                            ts = p;
                        }

                        _ => {}
                    }
                    _nacts -= 1;
                    _acts += 1;
                }
                if (p == eof) {
                    {
                        if (_khmer_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_khmer_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (_khmer_syllable_machine_key_offsets[(cs) as usize]) as i32;
                        _trans = (_khmer_syllable_machine_index_offsets[(cs) as usize]) as u32;
                        _klen = (_khmer_syllable_machine_single_lengths[(cs) as usize]) as i32;
                        __have = 0;
                        if (_klen > 0) {
                            {
                                let mut _lower: i32 = _keys;
                                let mut _upper: i32 = _keys + _klen - 1;
                                let mut _mid: i32 = 0;
                                while (true) {
                                    if (_upper < _lower) {
                                        {
                                            _keys += _klen;
                                            _trans += (_klen) as u32;
                                            break;
                                        }
                                    }
                                    _mid = _lower + ((_upper - _lower) >> 1);
                                    if ((buffer.info[p].khmer_category() as u8)
                                        < _khmer_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 1;
                                    } else if ((buffer.info[p].khmer_category() as u8)
                                        > _khmer_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _lower = _mid + 1;
                                    } else {
                                        {
                                            __have = 1;
                                            _trans += (_mid - _keys) as u32;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        _klen = (_khmer_syllable_machine_range_lengths[(cs) as usize]) as i32;
                        if (__have == 0 && _klen > 0) {
                            {
                                let mut _lower: i32 = _keys;
                                let mut _upper: i32 = _keys + (_klen << 1) - 2;
                                let mut _mid: i32 = 0;
                                while (true) {
                                    if (_upper < _lower) {
                                        {
                                            _trans += (_klen) as u32;
                                            break;
                                        }
                                    }
                                    _mid = _lower + (((_upper - _lower) >> 1) & !1);
                                    if ((buffer.info[p].khmer_category() as u8)
                                        < _khmer_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 2;
                                    } else if ((buffer.info[p].khmer_category() as u8)
                                        > _khmer_syllable_machine_trans_keys[(_mid + 1) as usize])
                                    {
                                        _lower = _mid + 2;
                                    } else {
                                        {
                                            _trans += ((_mid - _keys) >> 1) as u32;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                cs = (_khmer_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_khmer_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        _acts = (_khmer_syllable_machine_cond_actions[(_trans) as usize]) as i32;
                        _nacts = (_khmer_syllable_machine_actions[(_acts) as usize]) as u32;
                        _acts += 1;
                        while (_nacts > 0) {
                            match (_khmer_syllable_machine_actions[(_acts) as usize]) {
                                2 => {
                                    te = p + 1;
                                }
                                3 => {
                                    act = 2;
                                }
                                4 => {
                                    act = 3;
                                }
                                5 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::NonKhmerCluster);
                                    }
                                }
                                6 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::ConsonantSyllable);
                                    }
                                }
                                7 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::BrokenCluster);
                                        buffer.scratch_flags |=
                                            HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                    }
                                }
                                8 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::NonKhmerCluster);
                                    }
                                }
                                9 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::ConsonantSyllable);
                                    }
                                }
                                10 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::BrokenCluster);
                                        buffer.scratch_flags |=
                                            HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                    }
                                }
                                11 => match (act) {
                                    2 => {
                                        p = (te) - 1;
                                        {
                                            found_syllable!(SyllableType::BrokenCluster);
                                            buffer.scratch_flags |=
                                                HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                        }
                                    }
                                    3 => {
                                        p = (te) - 1;
                                        {
                                            found_syllable!(SyllableType::NonKhmerCluster);
                                        }
                                    }

                                    _ => {}
                                },

                                _ => {}
                            }
                            _nacts -= 1;
                            _acts += 1;
                        }
                    }
                }
                break '_again;
            }
            if (p == eof) {
                {
                    if (cs >= 21) {
                        break '_resume;
                    }
                }
            } else {
                {
                    _acts = (_khmer_syllable_machine_to_state_actions[(cs) as usize]) as i32;
                    _nacts = (_khmer_syllable_machine_actions[(_acts) as usize]) as u32;
                    _acts += 1;
                    while (_nacts > 0) {
                        match (_khmer_syllable_machine_actions[(_acts) as usize]) {
                            0 => {
                                ts = 0;
                            }

                            _ => {}
                        }
                        _nacts -= 1;
                        _acts += 1;
                    }
                    p += 1;
                    continue '_resume;
                }
            }
            break '_resume;
        }
    }
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &mut hb_buffer_t,
) {
    for i in start..end {
        buffer.info[i].set_syllable((*syllable_serial << 4) | kind as u8);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}
