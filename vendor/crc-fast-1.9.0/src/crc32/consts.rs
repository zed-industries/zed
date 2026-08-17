// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::consts::{
    NAME_CRC32_AIXM, NAME_CRC32_AUTOSAR, NAME_CRC32_BASE91_D, NAME_CRC32_BZIP2,
    NAME_CRC32_CD_ROM_EDC, NAME_CRC32_CKSUM, NAME_CRC32_ISCSI, NAME_CRC32_ISO_HDLC,
    NAME_CRC32_JAMCRC, NAME_CRC32_MEF, NAME_CRC32_MPEG_2, NAME_CRC32_XFER,
};
use crate::CrcAlgorithm;
use crate::CrcParams;
use crc::{
    CRC_32_AIXM, CRC_32_AUTOSAR, CRC_32_BASE91_D, CRC_32_BZIP2, CRC_32_CD_ROM_EDC, CRC_32_CKSUM,
    CRC_32_ISCSI, CRC_32_ISO_HDLC, CRC_32_JAMCRC, CRC_32_MEF, CRC_32_MPEG_2, CRC_32_XFER,
};

// width=32 poly=0x814141ab init=0x00000000 refin=false refout=false xorout=0x00000000 check=0x3010bf7f residue=0x00000000 name="CRC-32/AIXM"
pub const CRC32_AIXM: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Aixm,
    name: NAME_CRC32_AIXM,
    width: 32,
    poly: CRC_32_AIXM.poly as u64,
    init: CRC_32_AIXM.init as u64,
    init_algorithm: CRC_32_AIXM.init as u64,
    refin: CRC_32_AIXM.refin,   // false
    refout: CRC_32_AIXM.refout, // false
    xorout: CRC_32_AIXM.xorout as u64,
    check: CRC_32_AIXM.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_814141AB_FORWARD),
};

// width=32 poly=0xf4acfb13 init=0xffffffff refin=true refout=true xorout=0xffffffff check=0x1697d06a residue=0x904cddbf name="CRC-32/AUTOSAR"
pub const CRC32_AUTOSAR: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Autosar,
    name: NAME_CRC32_AUTOSAR,
    width: 32,
    poly: CRC_32_AUTOSAR.poly as u64,
    init: CRC_32_AUTOSAR.init as u64,
    init_algorithm: CRC_32_AUTOSAR.init as u64,
    refin: CRC_32_AUTOSAR.refin,   // true
    refout: CRC_32_AUTOSAR.refout, // true
    xorout: CRC_32_AUTOSAR.xorout as u64,
    check: CRC_32_AUTOSAR.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_F4ACFB13_REFLECTED),
};

// width=32 poly=0xa833982b init=0xffffffff refin=true refout=true xorout=0xffffffff check=0x87315576 residue=0x45270551 name="CRC-32/BASE91-D"
pub const CRC32_BASE91_D: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Base91D,
    name: NAME_CRC32_BASE91_D,
    width: 32,
    poly: CRC_32_BASE91_D.poly as u64,
    init: CRC_32_BASE91_D.init as u64,
    init_algorithm: CRC_32_BASE91_D.init as u64,
    refin: CRC_32_BASE91_D.refin,   // true
    refout: CRC_32_BASE91_D.refout, // true
    xorout: CRC_32_BASE91_D.xorout as u64,
    check: CRC_32_BASE91_D.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_A833982B_REFLECTED),
};

// width=32 poly=0x04c11db7 init=0xffffffff refin=false refout=false xorout=0xffffffff check=0xfc891918 residue=0xc704dd7b name="CRC-32/BZIP2"
pub const CRC32_BZIP2: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Bzip2,
    name: NAME_CRC32_BZIP2,
    width: 32,
    poly: CRC_32_BZIP2.poly as u64,
    init: CRC_32_BZIP2.init as u64,
    init_algorithm: CRC_32_BZIP2.init as u64,
    refin: CRC_32_BZIP2.refin,   // false
    refout: CRC_32_BZIP2.refout, // false
    xorout: CRC_32_BZIP2.xorout as u64,
    check: CRC_32_BZIP2.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_FORWARD),
};

// width=32 poly=0x8001801b init=0x00000000 refin=true refout=true xorout=0x00000000 check=0x6ec2edc4 residue=0x00000000 name="CRC-32/CD-ROM-EDC"
pub const CRC32_CD_ROM_EDC: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32CdRomEdc,
    name: NAME_CRC32_CD_ROM_EDC,
    width: 32,
    poly: CRC_32_CD_ROM_EDC.poly as u64,
    init: CRC_32_CD_ROM_EDC.init as u64,
    init_algorithm: CRC_32_CD_ROM_EDC.init as u64,
    refin: CRC_32_CD_ROM_EDC.refin,   // true
    refout: CRC_32_CD_ROM_EDC.refout, // true
    xorout: CRC_32_CD_ROM_EDC.xorout as u64,
    check: CRC_32_CD_ROM_EDC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_8001801B_REFLECTED),
};

// width=32 poly=0x04c11db7 init=0x00000000 refin=false refout=false xorout=0xffffffff check=0x765e7680 residue=0xc704dd7b name="CRC-32/CKSUM"
pub const CRC32_CKSUM: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Cksum,
    name: NAME_CRC32_CKSUM,
    width: 32,
    poly: CRC_32_CKSUM.poly as u64,
    init: CRC_32_CKSUM.init as u64,
    init_algorithm: CRC_32_CKSUM.init as u64,
    refin: CRC_32_CKSUM.refin,   // false
    refout: CRC_32_CKSUM.refout, // false
    xorout: CRC_32_CKSUM.xorout as u64,
    check: CRC_32_CKSUM.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_FORWARD),
};

// width=32 poly=0x1edc6f41 init=0xffffffff refin=true refout=true xorout=0xffffffff check=0xe3069283 residue=0xb798b438 name="CRC-32/ISCSI"
pub const CRC32_ISCSI: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Iscsi,
    name: NAME_CRC32_ISCSI,
    width: 32,
    poly: CRC_32_ISCSI.poly as u64,
    init: CRC_32_ISCSI.init as u64,
    init_algorithm: CRC_32_ISCSI.init as u64,
    refin: CRC_32_ISCSI.refin,   // true
    refout: CRC_32_ISCSI.refout, // true
    xorout: CRC_32_ISCSI.xorout as u64,
    check: CRC_32_ISCSI.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1EDC6F41_REFLECTED),
};

// width=32 poly=0x04c11db7 init=0xffffffff refin=true refout=true xorout=0xffffffff check=0xcbf43926 residue=0xdebb20e3 name="CRC-32/ISO-HDLC"
pub const CRC32_ISO_HDLC: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32IsoHdlc,
    name: NAME_CRC32_ISO_HDLC,
    width: 32,
    poly: CRC_32_ISO_HDLC.poly as u64,
    init: CRC_32_ISO_HDLC.init as u64,
    init_algorithm: CRC_32_ISO_HDLC.init as u64,
    refin: CRC_32_ISO_HDLC.refin,   // true
    refout: CRC_32_ISO_HDLC.refout, // true
    xorout: CRC_32_ISO_HDLC.xorout as u64,
    check: CRC_32_ISO_HDLC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_REFLECTED),
};

// width=32 poly=0x04c11db7 init=0xffffffff refin=true refout=true xorout=0x00000000 check=0x340bc6d9 residue=0x00000000 name="CRC-32/JAMCRC"
pub const CRC32_JAMCRC: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Jamcrc,
    name: NAME_CRC32_JAMCRC,
    width: 32,
    poly: CRC_32_JAMCRC.poly as u64,
    init: CRC_32_JAMCRC.init as u64,
    init_algorithm: CRC_32_JAMCRC.init as u64,
    refin: CRC_32_JAMCRC.refin,   // true
    refout: CRC_32_JAMCRC.refout, // true
    xorout: CRC_32_JAMCRC.xorout as u64,
    check: CRC_32_JAMCRC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_REFLECTED),
};

// width=32 poly=0x741b8cd7 init=0xffffffff refin=true refout=true xorout=0x00000000 check=0xd2c22f51 residue=0x00000000 name="CRC-32/MEF"
pub const CRC32_MEF: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Mef,
    name: NAME_CRC32_MEF,
    width: 32,
    poly: CRC_32_MEF.poly as u64,
    init: CRC_32_MEF.init as u64,
    init_algorithm: CRC_32_MEF.init as u64,
    refin: CRC_32_MEF.refin,   // true
    refout: CRC_32_MEF.refout, // true
    xorout: CRC_32_MEF.xorout as u64,
    check: CRC_32_MEF.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_741B8CD7_REFLECTED),
};

// width=32 poly=0x04c11db7 init=0xffffffff refin=false refout=false xorout=0x00000000 check=0x0376e6e7 residue=0x00000000 name="CRC-32/MPEG-2"
pub const CRC32_MPEG_2: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Mpeg2,
    name: NAME_CRC32_MPEG_2,
    width: 32,
    poly: CRC_32_MPEG_2.poly as u64,
    init: CRC_32_MPEG_2.init as u64,
    init_algorithm: CRC_32_MPEG_2.init as u64,
    refin: CRC_32_MPEG_2.refin,   // false
    refout: CRC_32_MPEG_2.refout, // false
    xorout: CRC_32_MPEG_2.xorout as u64,
    check: CRC_32_MPEG_2.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_04C11DB7_FORWARD),
};

// width=32 poly=0x000000af init=0x00000000 refin=false refout=false xorout=0x00000000 check=0xbd0be338 residue=0x00000000 name="CRC-32/XFER"
pub const CRC32_XFER: CrcParams = CrcParams {
    algorithm: CrcAlgorithm::Crc32Xfer,
    name: NAME_CRC32_XFER,
    width: 32,
    poly: CRC_32_XFER.poly as u64,
    init: CRC_32_XFER.init as u64,
    init_algorithm: CRC_32_XFER.init as u64,
    refin: CRC_32_XFER.refin,   // false
    refout: CRC_32_XFER.refout, // false
    xorout: CRC_32_XFER.xorout as u64,
    check: CRC_32_XFER.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_000000AF_FORWARD),
};

// CRC-32/AIXM
pub const KEYS_814141AB_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x9be9878f00000000,
    0x85b2a6e400000000,
    0x2aa81be300000000,
    0xa488a24c00000000,
    0x9be9878f00000000,
    0xb1efc5f600000000,
    0x00000001feff7f62,
    0x00000001814141ab,
    0x143b9cd200000000,
    0x9853011900000000,
    0x7836e63a00000000,
    0xaa29818100000000,
    0x3bd96ca700000000,
    0x60205cd400000000,
    0x74f21e8b00000000,
    0x3540871b00000000,
    0x0442099000000000,
    0x361f380200000000,
    0x6757ee2f00000000,
    0xffc42e7700000000,
    0xd12a88300000000,
    0x93a03b8800000000,
];

// CRC-32/AUTOSAR
pub const KEYS_F4ACFB13_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000016130902a,
    0x0000000050428a9c,
    0x000000010b1e9a08,
    0x00000000d77bb854,
    0x000000016130902a,
    0x00000001b0d566c0,
    0x000000013cfdbf23,
    0x0000000191be6a5f,
    0x00000000b105f098,
    0x00000001b260c18a,
    0x00000001b0d68118,
    0x00000000c6f0b5d2,
    0x00000000ce9a9f48,
    0x00000000fc24cbf6,
    0x0000000018c71228,
    0x000000014b462960,
    0x00000001848ecbce,
    0x0000000049cb6c68,
    0x00000000c9d55d76,
    0x0000000022919656,
    0x00000001e97b6a9e,
    0x00000000000cbd7c,
];

// CRC-32/BASE91-D
pub const KEYS_A833982B_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x00000001e065d896,
    0x00000001aca6d990,
    0x000000007ec6845e,
    0x000000009aa0f3be,
    0x00000001e065d896,
    0x00000000cf690ff2,
    0x000000009167fd37,
    0x00000001a833982b,
    0x00000001f0023e48,
    0x0000000054bacd0c,
    0x00000001677129ba,
    0x00000000ac52eee8,
    0x0000000068be1470,
    0x000000017208fc52,
    0x00000001c2e169fc,
    0x0000000122f9bd98,
    0x0000000192d6d10c,
    0x00000001942367fa,
    0x00000000c2044564,
    0x00000001a07ba234,
    0x000000010ffc58e6,
    0x000000015920d7a6,
];

// CRC-32/CD-ROM-EDC
pub const KEYS_8001801B_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x00000001d5934102,
    0x000000006c90c100,
    0x00000001fbea69a0,
    0x000000006500d000,
    0x00000001d5934102,
    0x00000001f1030002,
    0x000000017000ffff,
    0x00000001b0030003,
    0x0000000178be62fe,
    0x00000001353195ce,
    0x0000000085a25e78,
    0x00000000a23c9cc0,
    0x000000005ead8550,
    0x00000001eab75dd2,
    0x000000012e7928a2,
    0x00000001f8931102,
    0x0000000086acf0c0,
    0x00000001517f91c2,
    0x00000001f75a6182,
    0x00000000bd01c000,
    0x00000001bcb30820,
    0x000000010d925102,
];

// CRC-32/MEF
pub const KEYS_741B8CD7_REFLECTED: [u64; 23] = [
    0x0000000000000000,
    0x000000014b0602f8,
    0x000000007b4bc878,
    0x0000000023b08408,
    0x00000001e9bbe8a4,
    0x000000014b0602f8,
    0x0000000018c5564c,
    0x0000000017d232cd,
    0x00000001d663b05d,
    0x00000001f5dbe222,
    0x00000001290fe3ca,
    0x00000000048d6a82,
    0x0000000063b45844,
    0x00000001a9b7f536,
    0x0000000190afdbca,
    0x00000000be6d8f38,
    0x00000001c06a9816,
    0x00000001b5a46922,
    0x0000000097259f1a,
    0x00000000adfa5198,
    0x000000009c899030,
    0x00000001adf2908e,
    0x00000001f91b48f0,
];

// CRC-32/XFER
pub const KEYS_000000AF_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x00295f2300000000,
    0xfafa517900000000,
    0x5cd86bb500000000,
    0xaf6f37a300000000,
    0x00295f2300000000,
    0x0000445500000000,
    0x00000001000000af,
    0x00000001000000af,
    0x9bd57b5d00000000,
    0xb7a4d76400000000,
    0x1ae0004200000000,
    0xe7720be600000000,
    0x9c7fc8fe00000000,
    0x3885faf800000000,
    0xb477ad7100000000,
    0x0ac2ae3d00000000,
    0x5eae9dbe00000000,
    0x784a483800000000,
    0x7d21bf2000000000,
    0xfaebd3d300000000,
    0x25ed382b00000000,
    0x6d2b811a00000000,
];

// CRC-32/ISO-HDLC (aka 'crc32'), CRC-32/JAMCRC
const KEYS_04C11DB7_REFLECTED: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0x00000000ccaa009e, // (2^(32* 3) mod P(x))' << 1
    0x00000001751997d0, // (2^(32* 5) mod P(x))' << 1
    0x000000014a7fe880, // (2^(32*31) mod P(x))' << 1
    0x00000001e88ef372, // (2^(32*33) mod P(x))' << 1
    0x00000000ccaa009e, // (2^(32* 3) mod P(x))' << 1
    0x0000000163cd6124, // (2^(32* 2) mod P(x))' << 1
    0x00000001f7011641, // (floor(2^64/P(x)))'
    0x00000001db710641, // (P(x))'
    0x00000001d7cfc6ac, // (2^(32*27) mod P(x))' << 1
    0x00000001ea89367e, // (2^(32*29) mod P(x))' << 1
    0x000000018cb44e58, // (2^(32*23) mod P(x))' << 1
    0x00000000df068dc2, // (2^(32*25) mod P(x))' << 1
    0x00000000ae0b5394, // (2^(32*19) mod P(x))' << 1
    0x00000001c7569e54, // (2^(32*21) mod P(x))' << 1
    0x00000001c6e41596, // (2^(32*15) mod P(x))' << 1
    0x0000000154442bd4, // (2^(32*17) mod P(x))' << 1
    0x0000000174359406, // (2^(32*11) mod P(x))' << 1
    0x000000003db1ecdc, // (2^(32*13) mod P(x))' << 1
    0x000000015a546366, // (2^(32* 7) mod P(x))' << 1
    0x00000000f1da05aa, // (2^(32* 9) mod P(x))' << 1
    0x00000001322d1430,
    0x000000011542778a,
];

// CRC-32/ISCSI (aka 'crc32c')
const KEYS_1EDC6F41_REFLECTED: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0x000000014cd00bd6, // (2^(32* 3) mod P(x))' << 1
    0x00000000f20c0dfe, // (2^(32* 5) mod P(x))' << 1
    0x000000000d3b6092, // (2^(32*31) mod P(x))' << 1
    0x000000006992cea2, // (2^(32*33) mod P(x))' << 1
    0x000000014cd00bd6, // (2^(32* 3) mod P(x))' << 1
    0x00000000dd45aab8, // (2^(32* 2) mod P(x))' << 1
    0x00000000dea713f1, // (floor(2^64/P(x)))'
    0x0000000105ec76f1, // (P(x))'
    0x000000014237f5e6, // (2^(32*27) mod P(x))' << 1
    0x000000002ad91c30, // (2^(32*29) mod P(x))' << 1
    0x0000000102f9b8a2, // (2^(32*23) mod P(x))' << 1
    0x00000001c1733996, // (2^(32*25) mod P(x))' << 1
    0x0000000039d3b296, // (2^(32*19) mod P(x))' << 1
    0x00000000083a6eec, // (2^(32*21) mod P(x))' << 1
    0x000000009e4addf8, // (2^(32*15) mod P(x))' << 1
    0x00000000740eef02, // (2^(32*17) mod P(x))' << 1
    0x00000001d82c63da, // (2^(32*11) mod P(x))' << 1
    0x000000001c291d04, // (2^(32*13) mod P(x))' << 1
    0x00000000ba4fc28e, // (2^(32* 7) mod P(x))' << 1
    0x00000001384aa63a, // (2^(32* 9) mod P(x))' << 1
    0x00000000b9e02b86,
    0x00000000dcb17aa4,
];

// CRC-32/BZIP2, CRC-32/CKSUM, CRC-32/MPEG-2
const KEYS_04C11DB7_FORWARD: [u64; 23] = [
    0x0000000000000000, // unused placeholder to match 1-based indexing
    0xf200aa6600000000, // 2^(32* 3) mod P(x) << 32
    0x17d3315d00000000, // 2^(32* 5) mod P(x) << 32
    0x022ffca500000000, // 2^(32*31) mod P(x) << 32
    0x9d9ee22f00000000, // 2^(32*33) mod P(x) << 32
    0xf200aa6600000000, // 2^(32* 3) mod P(x) << 32
    0x490d678d00000000, // 2^(32* 2) mod P(x) << 32
    0x0000000104d101df, // floor(2^64/P(x))
    0x0000000104c11db7, // P(x)
    0x6ac7e7d700000000, // 2^(32*27) mod P(x) << 32
    0xfcd922af00000000, // 2^(32*29) mod P(x) << 32
    0x34e45a6300000000, // 2^(32*23) mod P(x) << 32
    0x8762c1f600000000, // 2^(32*25) mod P(x) << 32
    0x5395a0ea00000000, // 2^(32*19) mod P(x) << 32
    0x54f2d5c700000000, // 2^(32*21) mod P(x) << 32
    0xd3504ec700000000, // 2^(32*15) mod P(x) << 32
    0x57a8445500000000, // 2^(32*17) mod P(x) << 32
    0xc053585d00000000, // 2^(32*11) mod P(x) << 32
    0x766f1b7800000000, // 2^(32*13) mod P(x) << 32
    0xcd8c54b500000000, // 2^(32* 7) mod P(x) << 32
    0xab40b71e00000000, // 2^(32* 9) mod P(x) << 32
    0x1851689900000000,
    0xa3dc855100000000,
];

pub(crate) const SIMD_CONSTANTS: [[u64; 2]; 4] = [
    [0x08090a0b0c0d0e0f, 0x0001020304050607], // smask
    [0x8080808080808080, 0x8080808080808080], // mask1
    [0xffffffffffffffff, 0x00000000ffffffff], // mask2 forward
    [0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF], // mask2 reverse
];

pub(crate) const PSHUFB_SHF_TABLE_REVERSE: [[u64; 2]; 2] = [
    [0x0706050403020100, 0x0f0e0d0c0b0a0908],
    [0x8786858483828180, 0x8f8e8d8c8b8a8988],
];

pub(crate) const PSHUFB_SHF_TABLE_FORWARD: [[u64; 2]; 2] = [
    [0x8786858483828100, 0x8f8e8d8c8b8a8988],
    [0x0706050403020100, 0x0f0e0d0c0b0a0908],
];
