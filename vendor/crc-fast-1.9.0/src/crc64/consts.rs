// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::consts::*;
use crate::CrcAlgorithm;
use crate::CrcParams;
use crc::{CRC_64_ECMA_182, CRC_64_GO_ISO, CRC_64_MS, CRC_64_REDIS, CRC_64_WE, CRC_64_XZ};

// width=64 poly=0x42f0e1eba9ea3693 init=0x0000000000000000 refin=false refout=false xorout=0x0000000000000000 check=0x6c40df5f0b497347 residue=0x0000000000000000 name="CRC-64/ECMA-182"
pub const CRC64_ECMA_182: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64Ecma182,
    name: NAME_CRC64_ECMA_182,
    width: CRC_64_ECMA_182.width,
    poly: CRC_64_ECMA_182.poly,
    init: CRC_64_ECMA_182.init,
    init_algorithm: CRC_64_ECMA_182.init,
    refin: CRC_64_ECMA_182.refin,   // false
    refout: CRC_64_ECMA_182.refout, // false
    xorout: CRC_64_ECMA_182.xorout,
    check: CRC_64_ECMA_182.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_42F0E1EBA9EA3693_FORWARD),
};

// width=64 poly=0x000000000000001b init=0xffffffffffffffff refin=true refout=true xorout=0xffffffffffffffff check=0xb90956c775a41001 residue=0x5300000000000000 name="CRC-64/GO-ISO"
pub const CRC64_GO_ISO: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64GoIso,
    name: NAME_CRC64_GO_ISO,
    width: CRC_64_GO_ISO.width,
    poly: CRC_64_GO_ISO.poly,
    init: CRC_64_GO_ISO.init,
    init_algorithm: CRC_64_GO_ISO.init,
    refin: CRC_64_GO_ISO.refin,   // true
    refout: CRC_64_GO_ISO.refout, // true
    xorout: CRC_64_GO_ISO.xorout,
    check: CRC_64_GO_ISO.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_000000000000001B_REFLECTED),
};

// width=64 poly=0x259c84cba6426349 init=0xffffffffffffffff refin=true refout=true xorout=0x0000000000000000 check=0x75d4b74f024eceea residue=0x0000000000000000 name="CRC-64/MS"
pub const CRC64_MS: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64Ms,
    name: NAME_CRC64_MS,
    width: 64,
    poly: CRC_64_MS.poly,
    init: CRC_64_MS.init,
    init_algorithm: CRC_64_MS.init,
    refin: CRC_64_MS.refin,   // true
    refout: CRC_64_MS.refout, // true
    xorout: CRC_64_MS.xorout,
    check: CRC_64_MS.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_259C84CBA6426349_REFLECTED),
};

// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
// width=64 poly=0xad93d23594c93659 init=0xffffffffffffffff refin=true refout=true xorout=0xffffffffffffffff check=0xae8b14860a799888 residue=0xf310303b2b6f6e42 name="CRC-64/NVME"
pub const CRC64_NVME: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64Nvme,
    name: NAME_CRC64_NVME,
    width: CRC_64_NVME.width,
    poly: CRC_64_NVME.poly,
    init: CRC_64_NVME.init,
    init_algorithm: CRC_64_NVME.init,
    refin: CRC_64_NVME.refin,   // true
    refout: CRC_64_NVME.refout, // true
    xorout: CRC_64_NVME.xorout,
    check: CRC_64_NVME.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_AD93D23594C93659_REFLECTED),
};

// width=64 poly=0xad93d23594c935a9 init=0x0000000000000000 refin=true refout=true xorout=0x0000000000000000 check=0xe9c6d914c4b8d9ca residue=0x0000000000000000 name="CRC-64/REDIS"
pub const CRC64_REDIS: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64Redis,
    name: NAME_CRC64_REDIS,
    width: CRC_64_REDIS.width,
    poly: CRC_64_REDIS.poly,
    init: CRC_64_REDIS.init,
    init_algorithm: CRC_64_REDIS.init,
    refin: CRC_64_REDIS.refin,   // true
    refout: CRC_64_REDIS.refout, // true
    xorout: CRC_64_REDIS.xorout,
    check: CRC_64_REDIS.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_AD93D23594C935A9_REFLECTED),
};

// width=64 poly=0x42f0e1eba9ea3693 init=0xffffffffffffffff refin=false refout=false xorout=0xffffffffffffffff check=0x62ec59e3f1a4f00a residue=0xfcacbebd5931a992 name="CRC-64/WE"
pub const CRC64_WE: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64We,
    name: NAME_CRC64_WE,
    width: CRC_64_WE.width,
    poly: CRC_64_WE.poly,
    init: CRC_64_WE.init,
    init_algorithm: CRC_64_WE.init,
    refin: CRC_64_WE.refin,   // false
    refout: CRC_64_WE.refout, // false
    xorout: CRC_64_WE.xorout,
    check: CRC_64_WE.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_42F0E1EBA9EA3693_FORWARD),
};

// width=64 poly=0x42f0e1eba9ea3693 init=0xffffffffffffffff refin=true refout=true xorout=0xffffffffffffffff check=0x995dc9bbdf1939fa residue=0x49958c9abd7d353f name="CRC-64/XZ"
pub const CRC64_XZ: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc64Xz,
    name: NAME_CRC64_XZ,
    width: CRC_64_XZ.width,
    poly: CRC_64_XZ.poly,
    init: CRC_64_XZ.init,
    init_algorithm: CRC_64_XZ.init,
    refin: CRC_64_XZ.refin,   // true
    refout: CRC_64_XZ.refout, // true
    xorout: CRC_64_XZ.xorout,
    check: CRC_64_XZ.check,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_42F0E1EBA9EA3693_REFLECTED),
};

// CRC-64/MS
const KEYS_259C84CBA6426349_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0xcef05cca14bbf4df,
    0xfd5d7a0700b5ba38,
    0xafde70a30ebb4286,
    0xe7a651bf12fbb17b,
    0xcef05cca14bbf4df,
    0x0000000000000000,
    0xd7eb06822197a109,
    0x258c84cba6427349,
    0x9f5bbc2a0e2e4c6f,
    0x50e15bfbd337753a,
    0x064a94c5212d44f4,
    0x4aaa0531d46f3c70,
    0xebd4e5a2eb6d83a1,
    0x295b872fea8473f0,
    0xa62bc2d50bf03c03,
    0xd3e2dc3a51dacee1,
    0x4aa4564b4042092b,
    0x717984ed338c465f,
    0x70bd522114faceb8,
    0x2188097f5687b43c,
    0xb7c2f9fa47c4fe55,
    0x8dccaf9d6169d0fa,
];

// CRC-64/REDIS
const KEYS_AD93D23594C935A9_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x381d0015c96f4444,
    0xd9d7be7d505da32c,
    0x768361524d29ed0b,
    0xcc26fa7c57f8054c,
    0x381d0015c96f4444,
    0x0000000000000000,
    0x3e6cfa329aef9f77,
    0x2b5926535897936b,
    0x5bc94ba8e2087636,
    0x6cf09c8f37710b75,
    0x3885fd59e440d95a,
    0xbccba3936411fb7e,
    0xe4dd0d81cbfce585,
    0xb715e37b96ed8633,
    0xf49784a634f014e4,
    0xaf86efb16d9ab4fb,
    0x7b3211a760160db8,
    0xa062b2319d66692f,
    0xef3d1d18ed889ed2,
    0x6ba4d760ab38201e,
    0x9471a5389095fe44,
    0x9a8908341a6d6d52,
];

// CRC-64/ECMA-182, CRC-64/WE
const KEYS_42F0E1EBA9EA3693_FORWARD: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0x05f5c3c7eb52fab6, // 2^(64* 2) mod P(x)
    0x4eb938a7d257740e, // 2^(64* 3) mod P(x)
    0x05cf79dea9ac37d6, // 2^(64*16) mod P(x)
    0x001067e571d7d5c2, // 2^(64*17) mod P(x)
    0x05f5c3c7eb52fab6, // 2^(64* 2) mod P(x)
    0x0000000000000000, // 2^(64* 1) mod P(x)
    0x578d29d06cc4f872, // floor(2^128/P(x)) - 2^64, mu
    0x42f0e1eba9ea3693, // P(x) - 2^64, poly_simd
    0xe464f4df5fb60ac1, // 2^(64*14) mod P(x)
    0xb649c5b35a759cf2, // 2^(64*15) mod P(x)
    0x9af04e1eff82d0dd, // 2^(64*12) mod P(x)
    0x6e82e609297f8fe8, // 2^(64*13) mod P(x)
    0x097c516e98bd2e73, // 2^(64*10) mod P(x)
    0x0b76477b31e22e7b, // 2^(64*11) mod P(x)
    0x5f6843ca540df020, // 2^(64* 8) mod P(x)
    0xddf4b6981205b83f, // 2^(64* 9) mod P(x)
    0x54819d8713758b2c, // 2^(64* 6) mod P(x)
    0x4a6b90073eb0af5a, // 2^(64* 7) mod P(x)
    0x571bee0a227ef92b, // 2^(64* 4) mod P(x)
    0x44bef2a201b5200c, // 2^(64* 5) mod P(x)
    0x7f52691a60ddc70d,
    0x7036b0389f6a0c82,
];

// CRC-64/XZ
const KEYS_42F0E1EBA9EA3693_REFLECTED: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0xdabe95afc7875f40, // 2^((64* 2)-1) mod P(x)
    0xe05dd497ca393ae4, // 2^((64* 3)-1) mod P(x)
    0xd7d86b2af73de740, // 2^((64*16)-1) mod P(x)
    0x8757d71d4fcc1000, // 2^((64*17)-1) mod P(x)
    0xdabe95afc7875f40, // 2^((64* 2)-1) mod P(x)
    0x0000000000000000, // 2^((64* 1)-1) mod P(x)
    0x9c3e466c172963d5, // floor((2^127)/(P(x)), mu
    0x92d8af2baf0e1e85, // P(x) - 1, poly_simd
    0x947874de595052cb, // 2^((64*14)-1) mod P(x)
    0x9e735cb59b4724da, // 2^((64*15)-1) mod P(x)
    0xe4ce2cd55fea0037, // 2^((64*12)-1) mod P(x)
    0x2fe3fd2920ce82ec, // 2^((64*13)-1) mod P(x)
    0x0e31d519421a63a5, // 2^((64*10)-1) mod P(x)
    0x2e30203212cac325, // 2^((64*11)-1) mod P(x)
    0x081f6054a7842df4, // 2^((64* 8)-1) mod P(x)
    0x6ae3efbb9dd441f3, // 2^((64* 9)-1) mod P(x)
    0x69a35d91c3730254, // 2^((64* 6)-1) mod P(x)
    0xb5ea1af9c013aca4, // 2^((64* 7)-1) mod P(x)
    0x3be653a30fe1af51, // 2^((64* 4)-1) mod P(x)
    0x60095b008a9efa44, // 2^((64* 5)-1) mod P(x)
    0xf31fd9271e228b79,
    0x8260adf2381ad81c,
];

// CRC-64/GO-ISO
const KEYS_000000000000001B_REFLECTED: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0xf500000000000001, // 2^((64* 2)-1) mod P(x)
    0x6b70000000000001, // 2^((64* 3)-1) mod P(x)
    0xb001000000010000, // 2^((64*16)-1) mod P(x)
    0xf501b0000001b000, // 2^((64*17)-1) mod P(x)
    0xf500000000000001, // 2^((64* 2)-1) mod P(x)
    0x0000000000000000, // 2^((64* 1)-1) mod P(x)
    0xb000000000000001, // floor((2^127)/(P(x)), mu
    0xb000000000000001, // P(x) - 1, poly_simd
    0xe014514514501501, // 2^((64*14)-1) mod P(x)
    0x771db6db6db71c71, // 2^((64*15)-1) mod P(x)
    0xa101101101110001, // 2^((64*12)-1) mod P(x)
    0x1ab1ab1ab1aab001, // 2^((64*13)-1) mod P(x)
    0xf445014445000001, // 2^((64*10)-1) mod P(x)
    0x6aab71daab700001, // 2^((64*11)-1) mod P(x)
    0xb100010100000001, // 2^((64* 8)-1) mod P(x)
    0x01b001b1b0000001, // 2^((64* 9)-1) mod P(x)
    0xe145150000000001, // 2^((64* 6)-1) mod P(x)
    0x76db6c7000000001, // 2^((64* 7)-1) mod P(x)
    0xa011000000000001, // 2^((64* 4)-1) mod P(x)
    0x1b1ab00000000001, // 2^((64* 5)-1) mod P(x)
    0x45000000b0000000,
    0x6b700000f5000000,
];

// CRC-64/NVME
const KEYS_AD93D23594C93659_REFLECTED: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0x21e9_761e_2526_21ac,
    0xeadc_41fd_2ba3_d420,
    0x5f85_2fb6_1e8d_92dc,
    0xa1ca_681e_733f_9c40,
    0x21e9_761e_2526_21ac,
    0x0000000000000000,
    0x27ec_fa32_9aef_9f77, // mu
    0x34d9_2653_5897_936b, // poly
    0x9465_8840_3d4a_dcbc,
    0xd083_dd59_4d96_319d,
    0x34f5_a24e_22d6_6e90,
    0x3c25_5f5e_bc41_4423,
    0x0336_3823_e6e7_91e5,
    0x7b0a_b10d_d0f8_09fe,
    0x6224_2240_ace5_045a,
    0x0c32_cdb3_1e18_a84a,
    0xa3ff_dc1f_e8e8_2a8b,
    0xbdd7_ac0e_e1a4_a0f0,
    0xe1e0_bb9d_45d7_a44c,
    0xb0bc_2e58_9204_f500,
    0xa043_808c_0f78_2663,
    0x37cc_d3e1_4069_cabc,
];

pub const SIMD_CONSTANTS: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xffffffffffffffff, 0x00000000ffffffff], // mask2
    [0x0000000000000000, 0xffffffffffffffff], // mask3
];

/// Lookup tables for byte reflection and other operations
#[repr(C, align(16))]
pub struct AlignedTableReverse {
    row1: [u8; 16],
    row2: [u8; 16],
}

pub const PSBTBL_REVERSE: AlignedTableReverse = AlignedTableReverse {
    row1: [
        // First row (quadwords 1&2): 08786858483828100h, 08f8e8d8c8b8a8988h
        0x00, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
        0x8f,
    ],
    row2: [
        // Second row (quadwords 3&4): 00706050403020100h, 0000e0d0c0b0a0908h
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x00,
    ],
};

#[repr(C, align(16))]
pub struct AlignedTableForward {
    row1: [u8; 16],
    row2: [u8; 16],
    row3: [u8; 16],
    row4: [u8; 16],
}

pub const PSBTBL_FORWARD: AlignedTableForward = AlignedTableForward {
    // First row: 08786858483828100h, 08f8e8d8c8b8a8988h
    row1: [
        0x00, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
        0x8f,
    ],
    // Second row: 00706050403020100h, 00f0e0d0c0b0a0908h
    row2: [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ],
    // Third row: 08080808080808080h, 00f0e0d0c0b0a0908h
    row3: [
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ],
    // Fourth row: 08080808080808080h, 08080808080808080h
    row4: [
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80,
    ],
};
