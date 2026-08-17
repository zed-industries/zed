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

static _myanmar_syllable_machine_actions: [i8; 21] = [
    0, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 0, 0,
];
static _myanmar_syllable_machine_key_offsets: [i16; 57] = [
    0, 24, 42, 48, 51, 62, 69, 76, 81, 85, 93, 102, 112, 117, 120, 129, 137, 148, 158, 174, 186,
    197, 210, 223, 238, 252, 269, 275, 278, 289, 296, 303, 308, 312, 320, 329, 339, 344, 347, 365,
    374, 382, 393, 403, 419, 431, 442, 455, 468, 483, 497, 514, 532, 549, 572, 0, 0,
];
static _myanmar_syllable_machine_trans_keys: [u8; 579] = [
    3, 4, 8, 9, 15, 18, 20, 21, 22, 23, 32, 35, 36, 37, 38, 39, 40, 41, 1, 2, 5, 6, 10, 11, 3, 4,
    8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 38, 39, 40, 41, 5, 6, 8, 23, 32, 39, 5, 6, 8, 5, 6, 3, 8,
    9, 20, 23, 32, 35, 39, 41, 5, 6, 3, 8, 9, 23, 39, 5, 6, 3, 8, 9, 32, 39, 5, 6, 8, 32, 39, 5, 6,
    8, 39, 5, 6, 3, 8, 9, 20, 23, 39, 5, 6, 3, 8, 9, 20, 23, 32, 39, 5, 6, 3, 8, 9, 20, 23, 32, 39,
    41, 5, 6, 8, 23, 39, 5, 6, 15, 1, 2, 3, 8, 9, 20, 21, 23, 39, 5, 6, 3, 8, 9, 21, 23, 39, 5, 6,
    3, 8, 9, 20, 21, 22, 23, 39, 40, 5, 6, 3, 8, 9, 20, 21, 22, 23, 39, 5, 6, 3, 8, 9, 20, 21, 22,
    23, 32, 35, 36, 37, 38, 39, 41, 5, 6, 3, 8, 9, 20, 21, 22, 23, 32, 39, 41, 5, 6, 3, 8, 9, 20,
    21, 22, 23, 32, 39, 5, 6, 3, 8, 9, 20, 21, 22, 23, 35, 37, 39, 41, 5, 6, 3, 8, 9, 20, 21, 22,
    23, 32, 35, 39, 41, 5, 6, 3, 8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 39, 41, 5, 6, 3, 8, 9, 20,
    21, 22, 23, 35, 36, 37, 39, 41, 5, 6, 3, 4, 8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 38, 39, 41,
    5, 6, 8, 23, 32, 39, 5, 6, 8, 5, 6, 3, 8, 9, 20, 23, 32, 35, 39, 41, 5, 6, 3, 8, 9, 23, 39, 5,
    6, 3, 8, 9, 32, 39, 5, 6, 8, 32, 39, 5, 6, 8, 39, 5, 6, 3, 8, 9, 20, 23, 39, 5, 6, 3, 8, 9, 20,
    23, 32, 39, 5, 6, 3, 8, 9, 20, 23, 32, 39, 41, 5, 6, 8, 23, 39, 5, 6, 15, 1, 2, 3, 4, 8, 9, 20,
    21, 22, 23, 32, 35, 36, 37, 38, 39, 40, 41, 5, 6, 3, 8, 9, 20, 21, 23, 39, 5, 6, 3, 8, 9, 21,
    23, 39, 5, 6, 3, 8, 9, 20, 21, 22, 23, 39, 40, 5, 6, 3, 8, 9, 20, 21, 22, 23, 39, 5, 6, 3, 8,
    9, 20, 21, 22, 23, 32, 35, 36, 37, 38, 39, 41, 5, 6, 3, 8, 9, 20, 21, 22, 23, 32, 39, 41, 5, 6,
    3, 8, 9, 20, 21, 22, 23, 32, 39, 5, 6, 3, 8, 9, 20, 21, 22, 23, 35, 37, 39, 41, 5, 6, 3, 8, 9,
    20, 21, 22, 23, 32, 35, 39, 41, 5, 6, 3, 8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 39, 41, 5, 6, 3,
    8, 9, 20, 21, 22, 23, 35, 36, 37, 39, 41, 5, 6, 3, 4, 8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 38,
    39, 41, 5, 6, 3, 4, 8, 9, 20, 21, 22, 23, 32, 35, 36, 37, 38, 39, 40, 41, 5, 6, 3, 4, 8, 9, 20,
    21, 22, 23, 32, 35, 36, 37, 38, 39, 41, 5, 6, 3, 4, 8, 9, 15, 20, 21, 22, 23, 32, 35, 36, 37,
    38, 39, 40, 41, 1, 2, 5, 6, 10, 11, 15, 1, 2, 10, 11, 0, 0,
];
static _myanmar_syllable_machine_single_lengths: [i8; 57] = [
    18, 16, 4, 1, 9, 5, 5, 3, 2, 6, 7, 8, 3, 1, 7, 6, 9, 8, 14, 10, 9, 11, 11, 13, 12, 15, 4, 1, 9,
    5, 5, 3, 2, 6, 7, 8, 3, 1, 16, 7, 6, 9, 8, 14, 10, 9, 11, 11, 13, 12, 15, 16, 15, 17, 1, 0, 0,
];
static _myanmar_syllable_machine_range_lengths: [i8; 57] = [
    3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 0, 0,
];
static _myanmar_syllable_machine_index_offsets: [i16; 57] = [
    0, 22, 40, 46, 49, 60, 67, 74, 79, 83, 91, 100, 110, 115, 118, 127, 135, 146, 156, 172, 184,
    195, 208, 221, 236, 250, 267, 273, 276, 287, 294, 301, 306, 310, 318, 327, 337, 342, 345, 363,
    372, 380, 391, 401, 417, 429, 440, 453, 466, 481, 495, 512, 530, 547, 568, 0, 0,
];
static _myanmar_syllable_machine_cond_targs: [i8; 629] = [
    26, 37, 27, 29, 51, 54, 39, 40, 41, 28, 43, 44, 46, 47, 48, 30, 50, 45, 1, 0, 1, 0, 2, 13, 3,
    5, 14, 15, 16, 4, 18, 19, 21, 22, 23, 6, 25, 20, 0, 0, 3, 4, 12, 6, 0, 0, 3, 0, 0, 2, 3, 5, 9,
    4, 10, 11, 6, 10, 0, 0, 2, 3, 5, 4, 6, 0, 0, 7, 3, 6, 8, 6, 0, 0, 3, 8, 6, 0, 0, 3, 6, 0, 0, 2,
    3, 5, 9, 4, 6, 0, 0, 2, 3, 5, 9, 4, 10, 6, 0, 0, 2, 3, 5, 9, 4, 10, 6, 10, 0, 0, 3, 4, 6, 0, 0,
    1, 1, 0, 2, 3, 5, 14, 15, 4, 6, 0, 0, 2, 3, 5, 15, 4, 6, 0, 0, 2, 3, 5, 14, 15, 16, 4, 6, 17,
    0, 0, 2, 3, 5, 14, 15, 16, 4, 6, 0, 0, 2, 3, 5, 14, 15, 16, 4, 18, 19, 21, 22, 23, 6, 20, 0, 0,
    2, 3, 5, 14, 15, 16, 4, 17, 6, 20, 0, 0, 2, 3, 5, 14, 15, 16, 4, 17, 6, 0, 0, 2, 3, 5, 14, 15,
    16, 4, 19, 22, 6, 20, 0, 0, 2, 3, 5, 14, 15, 16, 4, 17, 19, 6, 20, 0, 0, 2, 3, 5, 14, 15, 16,
    4, 24, 19, 21, 22, 6, 20, 0, 0, 2, 3, 5, 14, 15, 16, 4, 19, 21, 22, 6, 20, 0, 0, 2, 13, 3, 5,
    14, 15, 16, 4, 18, 19, 21, 22, 23, 6, 20, 0, 0, 27, 28, 36, 30, 0, 0, 27, 0, 0, 26, 27, 29, 33,
    28, 34, 35, 30, 34, 0, 0, 26, 27, 29, 28, 30, 0, 0, 31, 27, 30, 32, 30, 0, 0, 27, 32, 30, 0, 0,
    27, 30, 0, 0, 26, 27, 29, 33, 28, 30, 0, 0, 26, 27, 29, 33, 28, 34, 30, 0, 0, 26, 27, 29, 33,
    28, 34, 30, 34, 0, 0, 27, 28, 30, 0, 0, 38, 38, 0, 26, 37, 27, 29, 39, 40, 41, 28, 43, 44, 46,
    47, 48, 30, 50, 45, 0, 0, 26, 27, 29, 39, 40, 28, 30, 0, 0, 26, 27, 29, 40, 28, 30, 0, 0, 26,
    27, 29, 39, 40, 41, 28, 30, 42, 0, 0, 26, 27, 29, 39, 40, 41, 28, 30, 0, 0, 26, 27, 29, 39, 40,
    41, 28, 43, 44, 46, 47, 48, 30, 45, 0, 0, 26, 27, 29, 39, 40, 41, 28, 42, 30, 45, 0, 0, 26, 27,
    29, 39, 40, 41, 28, 42, 30, 0, 0, 26, 27, 29, 39, 40, 41, 28, 44, 47, 30, 45, 0, 0, 26, 27, 29,
    39, 40, 41, 28, 42, 44, 30, 45, 0, 0, 26, 27, 29, 39, 40, 41, 28, 49, 44, 46, 47, 30, 45, 0, 0,
    26, 27, 29, 39, 40, 41, 28, 44, 46, 47, 30, 45, 0, 0, 26, 37, 27, 29, 39, 40, 41, 28, 43, 44,
    46, 47, 48, 30, 45, 0, 0, 2, 13, 3, 5, 14, 15, 16, 4, 52, 19, 21, 22, 23, 6, 25, 20, 0, 0, 2,
    53, 3, 5, 14, 15, 16, 4, 18, 19, 21, 22, 23, 6, 20, 0, 0, 26, 37, 27, 29, 1, 39, 40, 41, 28,
    43, 44, 46, 47, 48, 30, 50, 45, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _myanmar_syllable_machine_cond_actions: [i8; 629] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 5, 13, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0,
    0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 5, 13, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 5, 13,
    0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 5, 13, 0, 0, 13, 0, 0, 0,
    0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0,
    0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
    13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 9, 15, 0, 9,
    15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 9,
    15, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0,
    9, 15, 0, 0, 0, 9, 15, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0,
    0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0,
    0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 15, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 13, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 15, 0, 0, 0, 17, 0, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 13, 13, 15, 17, 0,
    0,
];
static _myanmar_syllable_machine_to_state_actions: [i8; 57] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _myanmar_syllable_machine_from_state_actions: [i8; 57] = [
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _myanmar_syllable_machine_eof_trans: [i16; 57] = [
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623, 624, 625, 626, 627, 0, 0,
];
static myanmar_syllable_machine_start: i32 = 0;
static myanmar_syllable_machine_first_final: i32 = 0;
static myanmar_syllable_machine_error: i32 = -1;
static myanmar_syllable_machine_en_main: i32 = 0;
#[derive(Clone, Copy)]
pub enum SyllableType {
    ConsonantSyllable = 0,
    PunctuationCluster,
    BrokenCluster,
    NonMyanmarCluster,
}

pub fn find_syllables_myanmar(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let mut ts = 0;
    let mut te;
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
        cs = (myanmar_syllable_machine_start) as i32;
        ts = 0;
        te = 0;
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
                _acts = (_myanmar_syllable_machine_from_state_actions[(cs) as usize]) as i32;
                _nacts = (_myanmar_syllable_machine_actions[(_acts) as usize]) as u32;
                _acts += 1;
                while (_nacts > 0) {
                    match (_myanmar_syllable_machine_actions[(_acts) as usize]) {
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
                        if (_myanmar_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_myanmar_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (_myanmar_syllable_machine_key_offsets[(cs) as usize]) as i32;
                        _trans = (_myanmar_syllable_machine_index_offsets[(cs) as usize]) as u32;
                        _klen = (_myanmar_syllable_machine_single_lengths[(cs) as usize]) as i32;
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
                                    if ((buffer.info[p].myanmar_category() as u8)
                                        < _myanmar_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 1;
                                    } else if ((buffer.info[p].myanmar_category() as u8)
                                        > _myanmar_syllable_machine_trans_keys[(_mid) as usize])
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
                        _klen = (_myanmar_syllable_machine_range_lengths[(cs) as usize]) as i32;
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
                                    if ((buffer.info[p].myanmar_category() as u8)
                                        < _myanmar_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 2;
                                    } else if ((buffer.info[p].myanmar_category() as u8)
                                        > _myanmar_syllable_machine_trans_keys[(_mid + 1) as usize])
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
                cs = (_myanmar_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_myanmar_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        _acts = (_myanmar_syllable_machine_cond_actions[(_trans) as usize]) as i32;
                        _nacts = (_myanmar_syllable_machine_actions[(_acts) as usize]) as u32;
                        _acts += 1;
                        while (_nacts > 0) {
                            match (_myanmar_syllable_machine_actions[(_acts) as usize]) {
                                2 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::ConsonantSyllable);
                                    }
                                }
                                3 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::NonMyanmarCluster);
                                    }
                                }
                                4 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::BrokenCluster);
                                        buffer.scratch_flags |=
                                            HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                    }
                                }
                                5 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::NonMyanmarCluster);
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
                                        found_syllable!(SyllableType::NonMyanmarCluster);
                                    }
                                }

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
                    if (cs >= 0) {
                        break '_resume;
                    }
                }
            } else {
                {
                    _acts = (_myanmar_syllable_machine_to_state_actions[(cs) as usize]) as i32;
                    _nacts = (_myanmar_syllable_machine_actions[(_acts) as usize]) as u32;
                    _acts += 1;
                    while (_nacts > 0) {
                        match (_myanmar_syllable_machine_actions[(_acts) as usize]) {
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
