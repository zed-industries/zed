// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![cfg(test)]
#![allow(dead_code)]

use crate::consts::CRC_64_NVME;
use crate::crc16::consts::{
    CRC16_ARC, CRC16_CDMA2000, CRC16_CMS, CRC16_DDS_110, CRC16_DECT_R, CRC16_DECT_X, CRC16_DNP,
    CRC16_EN_13757, CRC16_GENIBUS, CRC16_GSM, CRC16_IBM_3740, CRC16_IBM_SDLC,
    CRC16_ISO_IEC_14443_3_A, CRC16_KERMIT, CRC16_LJ1200, CRC16_M17, CRC16_MAXIM_DOW, CRC16_MCRF4XX,
    CRC16_MODBUS, CRC16_NRSC_5, CRC16_OPENSAFETY_A, CRC16_OPENSAFETY_B, CRC16_PROFIBUS,
    CRC16_RIELLO, CRC16_SPI_FUJITSU, CRC16_T10_DIF, CRC16_TELEDISK, CRC16_TMS37157, CRC16_UMTS,
    CRC16_USB, CRC16_XMODEM,
};
use crate::crc32::consts::{
    CRC32_AIXM, CRC32_AUTOSAR, CRC32_BASE91_D, CRC32_BZIP2, CRC32_CD_ROM_EDC, CRC32_CKSUM,
    CRC32_ISCSI, CRC32_ISO_HDLC, CRC32_JAMCRC, CRC32_MEF, CRC32_MPEG_2, CRC32_XFER,
};
use crate::crc64::consts::{
    CRC64_ECMA_182, CRC64_GO_ISO, CRC64_MS, CRC64_NVME, CRC64_REDIS, CRC64_WE, CRC64_XZ,
};
use crate::test::enums::*;
use crate::test::structs::*;
use crc::Table;

pub const TEST_CHECK_STRING: &[u8] = b"123456789";

pub const TEST_256_BYTES_STRING: &[u8] = b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456";

pub const TEST_255_BYTES_STRING: &[u8] = b"123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345";

pub(crate) const RUST_CRC32_AIXM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AIXM);

pub(crate) const RUST_CRC32_AUTOSAR: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AUTOSAR);

pub(crate) const RUST_CRC32_BASE91_D: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BASE91_D);

pub(crate) const RUST_CRC32_BZIP2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BZIP2);

pub(crate) const RUST_CRC32_CD_ROM_EDC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CD_ROM_EDC);

pub(crate) const RUST_CRC32_CKSUM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CKSUM);

pub(crate) const RUST_CRC32_ISCSI: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISCSI);

pub(crate) const RUST_CRC32_ISO_HDLC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISO_HDLC);

pub(crate) const RUST_CRC32_JAMCRC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_JAMCRC);

pub(crate) const RUST_CRC32_MEF: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MEF);

pub(crate) const RUST_CRC32_MPEG_2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MPEG_2);

pub(crate) const RUST_CRC32_XFER: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_XFER);

pub(crate) const RUST_CRC16_ARC: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_ARC);

pub(crate) const RUST_CRC16_CDMA2000: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_CDMA2000);

pub(crate) const RUST_CRC16_CMS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_CMS);

pub(crate) const RUST_CRC16_DDS_110: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DDS_110);

pub(crate) const RUST_CRC16_DECT_R: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DECT_R);

pub(crate) const RUST_CRC16_DECT_X: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DECT_X);

pub(crate) const RUST_CRC16_DNP: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DNP);

pub(crate) const RUST_CRC16_EN_13757: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_EN_13757);

pub(crate) const RUST_CRC16_GENIBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_GENIBUS);

pub(crate) const RUST_CRC16_GSM: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_GSM);

pub(crate) const RUST_CRC16_IBM_3740: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_IBM_3740);

pub(crate) const RUST_CRC16_IBM_SDLC: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_IBM_SDLC);

pub(crate) const RUST_CRC16_ISO_IEC_14443_3_A: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_ISO_IEC_14443_3_A);

pub(crate) const RUST_CRC16_KERMIT: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_KERMIT);

pub(crate) const RUST_CRC16_LJ1200: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_LJ1200);

pub(crate) const RUST_CRC16_M17: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_M17);

pub(crate) const RUST_CRC16_MAXIM_DOW: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MAXIM_DOW);

pub(crate) const RUST_CRC16_MCRF4XX: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MCRF4XX);

pub(crate) const RUST_CRC16_MODBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MODBUS);

pub(crate) const RUST_CRC16_NRSC_5: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_NRSC_5);

pub(crate) const RUST_CRC16_OPENSAFETY_A: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_OPENSAFETY_A);

pub(crate) const RUST_CRC16_OPENSAFETY_B: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_OPENSAFETY_B);

pub(crate) const RUST_CRC16_PROFIBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_PROFIBUS);

pub(crate) const RUST_CRC16_RIELLO: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_RIELLO);

pub(crate) const RUST_CRC16_SPI_FUJITSU: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_SPI_FUJITSU);

pub(crate) const RUST_CRC16_T10_DIF: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_T10_DIF);

pub(crate) const RUST_CRC16_TELEDISK: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_TELEDISK);

pub(crate) const RUST_CRC16_TMS37157: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_TMS37157);

pub(crate) const RUST_CRC16_UMTS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_UMTS);

pub(crate) const RUST_CRC16_USB: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_USB);

pub(crate) const RUST_CRC16_XMODEM: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_XMODEM);

pub(crate) const RUST_CRC64_ECMA_182: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_ECMA_182);

pub(crate) const RUST_CRC64_GO_ISO: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_GO_ISO);

pub(crate) const RUST_CRC64_MS: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_MS);

pub(crate) const RUST_CRC64_NVME: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&CRC_64_NVME);

pub(crate) const RUST_CRC64_REDIS: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_REDIS);

pub(crate) const RUST_CRC64_WE: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_WE);

pub(crate) const RUST_CRC64_XZ: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_XZ);

pub(crate) const TEST_CRC64_ECMA_182: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_ECMA_182,
    reference_impl: &RUST_CRC64_ECMA_182,
};

pub(crate) const TEST_CRC64_GO_ISO: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_GO_ISO,
    reference_impl: &RUST_CRC64_GO_ISO,
};

pub(crate) const TEST_CRC64_MS: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_MS,
    reference_impl: &RUST_CRC64_MS,
};

pub(crate) const TEST_CRC64_NVME: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_NVME,
    reference_impl: &RUST_CRC64_NVME,
};

pub(crate) const TEST_CRC64_REDIS: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_REDIS,
    reference_impl: &RUST_CRC64_REDIS,
};

pub(crate) const TEST_CRC64_WE: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_WE,
    reference_impl: &RUST_CRC64_WE,
};

pub(crate) const TEST_CRC64_XZ: Crc64TestConfig = Crc64TestConfig {
    params: CRC64_XZ,
    reference_impl: &RUST_CRC64_XZ,
};

pub(crate) const TEST_CRC16_ARC: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_ARC,
    reference_impl: &RUST_CRC16_ARC,
};

pub(crate) const TEST_CRC16_CDMA2000: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_CDMA2000,
    reference_impl: &RUST_CRC16_CDMA2000,
};

pub(crate) const TEST_CRC16_CMS: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_CMS,
    reference_impl: &RUST_CRC16_CMS,
};

pub(crate) const TEST_CRC16_DDS_110: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_DDS_110,
    reference_impl: &RUST_CRC16_DDS_110,
};

pub(crate) const TEST_CRC16_DECT_R: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_DECT_R,
    reference_impl: &RUST_CRC16_DECT_R,
};

pub(crate) const TEST_CRC16_DECT_X: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_DECT_X,
    reference_impl: &RUST_CRC16_DECT_X,
};

pub(crate) const TEST_CRC16_DNP: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_DNP,
    reference_impl: &RUST_CRC16_DNP,
};

pub(crate) const TEST_CRC16_EN_13757: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_EN_13757,
    reference_impl: &RUST_CRC16_EN_13757,
};

pub(crate) const TEST_CRC16_GENIBUS: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_GENIBUS,
    reference_impl: &RUST_CRC16_GENIBUS,
};

pub(crate) const TEST_CRC16_GSM: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_GSM,
    reference_impl: &RUST_CRC16_GSM,
};

pub(crate) const TEST_CRC16_IBM_3740: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_IBM_3740,
    reference_impl: &RUST_CRC16_IBM_3740,
};

pub(crate) const TEST_CRC16_IBM_SDLC: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_IBM_SDLC,
    reference_impl: &RUST_CRC16_IBM_SDLC,
};

pub(crate) const TEST_CRC16_ISO_IEC_14443_3_A: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_ISO_IEC_14443_3_A,
    reference_impl: &RUST_CRC16_ISO_IEC_14443_3_A,
};

pub(crate) const TEST_CRC16_KERMIT: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_KERMIT,
    reference_impl: &RUST_CRC16_KERMIT,
};

pub(crate) const TEST_CRC16_LJ1200: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_LJ1200,
    reference_impl: &RUST_CRC16_LJ1200,
};

pub(crate) const TEST_CRC16_M17: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_M17,
    reference_impl: &RUST_CRC16_M17,
};

pub(crate) const TEST_CRC16_MAXIM_DOW: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_MAXIM_DOW,
    reference_impl: &RUST_CRC16_MAXIM_DOW,
};

pub(crate) const TEST_CRC16_MCRF4XX: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_MCRF4XX,
    reference_impl: &RUST_CRC16_MCRF4XX,
};

pub(crate) const TEST_CRC16_MODBUS: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_MODBUS,
    reference_impl: &RUST_CRC16_MODBUS,
};

pub(crate) const TEST_CRC16_NRSC_5: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_NRSC_5,
    reference_impl: &RUST_CRC16_NRSC_5,
};

pub(crate) const TEST_CRC16_OPENSAFETY_A: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_OPENSAFETY_A,
    reference_impl: &RUST_CRC16_OPENSAFETY_A,
};

pub(crate) const TEST_CRC16_OPENSAFETY_B: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_OPENSAFETY_B,
    reference_impl: &RUST_CRC16_OPENSAFETY_B,
};

pub(crate) const TEST_CRC16_PROFIBUS: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_PROFIBUS,
    reference_impl: &RUST_CRC16_PROFIBUS,
};

pub(crate) const TEST_CRC16_RIELLO: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_RIELLO,
    reference_impl: &RUST_CRC16_RIELLO,
};

pub(crate) const TEST_CRC16_SPI_FUJITSU: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_SPI_FUJITSU,
    reference_impl: &RUST_CRC16_SPI_FUJITSU,
};

pub(crate) const TEST_CRC16_T10_DIF: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_T10_DIF,
    reference_impl: &RUST_CRC16_T10_DIF,
};

pub(crate) const TEST_CRC16_TELEDISK: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_TELEDISK,
    reference_impl: &RUST_CRC16_TELEDISK,
};

pub(crate) const TEST_CRC16_TMS37157: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_TMS37157,
    reference_impl: &RUST_CRC16_TMS37157,
};

pub(crate) const TEST_CRC16_UMTS: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_UMTS,
    reference_impl: &RUST_CRC16_UMTS,
};

pub(crate) const TEST_CRC16_USB: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_USB,
    reference_impl: &RUST_CRC16_USB,
};

pub(crate) const TEST_CRC16_XMODEM: Crc16TestConfig = Crc16TestConfig {
    params: CRC16_XMODEM,
    reference_impl: &RUST_CRC16_XMODEM,
};

pub(crate) const TEST_CRC32_AIXM: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_AIXM,
    reference_impl: &RUST_CRC32_AIXM,
};

pub(crate) const TEST_CRC32_AUTOSAR: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_AUTOSAR,
    reference_impl: &RUST_CRC32_AUTOSAR,
};

pub(crate) const TEST_CRC32_BASE91_D: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_BASE91_D,
    reference_impl: &RUST_CRC32_BASE91_D,
};

pub(crate) const TEST_CRC32_BZIP2: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_BZIP2,
    reference_impl: &RUST_CRC32_BZIP2,
};

pub(crate) const TEST_CRC32_CD_ROM_EDC: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_CD_ROM_EDC,
    reference_impl: &RUST_CRC32_CD_ROM_EDC,
};

pub(crate) const TEST_CRC32_CKSUM: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_CKSUM,
    reference_impl: &RUST_CRC32_CKSUM,
};

pub(crate) const TEST_CRC32_ISCSI: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_ISCSI,
    reference_impl: &RUST_CRC32_ISCSI,
};

pub(crate) const TEST_CRC32_ISO_HDLC: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_ISO_HDLC,
    reference_impl: &RUST_CRC32_ISO_HDLC,
};

pub(crate) const TEST_CRC32_JAMCRC: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_JAMCRC,
    reference_impl: &RUST_CRC32_JAMCRC,
};

pub(crate) const TEST_CRC32_MEF: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_MEF,
    reference_impl: &RUST_CRC32_MEF,
};

pub(crate) const TEST_CRC32_MPEG_2: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_MPEG_2,
    reference_impl: &RUST_CRC32_MPEG_2,
};

pub(crate) const TEST_CRC32_XFER: Crc32TestConfig = Crc32TestConfig {
    params: CRC32_XFER,
    reference_impl: &RUST_CRC32_XFER,
};

pub(crate) const TEST_ALL_CONFIGS: &[AnyCrcTestConfig] = &[
    AnyCrcTestConfig::CRC16(&TEST_CRC16_ARC),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_CDMA2000),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_CMS),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_DDS_110),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_DECT_R),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_DECT_X),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_DNP),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_EN_13757),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_GENIBUS),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_GSM),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_IBM_3740),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_IBM_SDLC),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_ISO_IEC_14443_3_A),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_KERMIT),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_LJ1200),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_M17),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_MAXIM_DOW),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_MCRF4XX),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_MODBUS),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_NRSC_5),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_OPENSAFETY_A),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_OPENSAFETY_B),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_PROFIBUS),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_RIELLO),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_SPI_FUJITSU),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_T10_DIF),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_TELEDISK),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_TMS37157),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_UMTS),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_USB),
    AnyCrcTestConfig::CRC16(&TEST_CRC16_XMODEM),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_AIXM),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_AUTOSAR),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_BASE91_D),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_BZIP2),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_CD_ROM_EDC),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_CKSUM),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_ISCSI),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_ISO_HDLC),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_JAMCRC),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_MEF),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_MPEG_2),
    AnyCrcTestConfig::CRC32(&TEST_CRC32_XFER),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_ECMA_182),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_GO_ISO),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_MS),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_NVME),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_REDIS),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_WE),
    AnyCrcTestConfig::CRC64(&TEST_CRC64_XZ),
];
