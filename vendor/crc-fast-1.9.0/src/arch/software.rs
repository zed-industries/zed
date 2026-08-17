// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module contains a software fallback for unsupported architectures.

use crate::consts::CRC_64_NVME;
use crate::CrcAlgorithm;
use crate::CrcParams;
#[cfg(feature = "alloc")]
use crc::Algorithm;
use crc::Table;

// Caching for custom CRC algorithms to prevent repeated memory leaks
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use hashbrown::HashMap;
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use spin::{Mutex, Once};

// Cache key types for custom algorithms
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc16Key = (u16, u16, bool, bool, u16, u16);
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc32Key = (u32, u32, bool, bool, u32, u32);
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc64Key = (u64, u64, bool, bool, u64, u64);

// Global caches for custom algorithms (std version)
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC16_CACHE: OnceLock<Mutex<HashMap<Crc16Key, &'static Algorithm<u16>>>> =
    OnceLock::new();
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC32_CACHE: OnceLock<Mutex<HashMap<Crc32Key, &'static Algorithm<u32>>>> =
    OnceLock::new();
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC64_CACHE: OnceLock<Mutex<HashMap<Crc64Key, &'static Algorithm<u64>>>> =
    OnceLock::new();

// Global caches for custom algorithms (no_std + cache version)
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC16_CACHE: Once<Mutex<HashMap<Crc16Key, &'static Algorithm<u16>>>> = Once::new();
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC32_CACHE: Once<Mutex<HashMap<Crc32Key, &'static Algorithm<u32>>>> = Once::new();
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC64_CACHE: Once<Mutex<HashMap<Crc64Key, &'static Algorithm<u64>>>> = Once::new();

#[allow(unused)]
const RUST_CRC16_ARC: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_ARC);

#[allow(unused)]
const RUST_CRC16_CDMA2000: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_CDMA2000);

#[allow(unused)]
const RUST_CRC16_CMS: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_CMS);

#[allow(unused)]
const RUST_CRC16_DDS_110: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DDS_110);

#[allow(unused)]
const RUST_CRC16_DECT_R: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DECT_R);

#[allow(unused)]
const RUST_CRC16_DECT_X: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DECT_X);

#[allow(unused)]
const RUST_CRC16_DNP: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_DNP);

#[allow(unused)]
const RUST_CRC16_EN_13757: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_EN_13757);

#[allow(unused)]
const RUST_CRC16_GENIBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_GENIBUS);

#[allow(unused)]
const RUST_CRC16_GSM: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_GSM);

#[allow(unused)]
const RUST_CRC16_IBM_3740: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_IBM_3740);

#[allow(unused)]
const RUST_CRC16_IBM_SDLC: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_IBM_SDLC);

#[allow(unused)]
const RUST_CRC16_ISO_IEC_14443_3_A: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_ISO_IEC_14443_3_A);

#[allow(unused)]
const RUST_CRC16_KERMIT: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_KERMIT);

#[allow(unused)]
const RUST_CRC16_LJ1200: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_LJ1200);

#[allow(unused)]
const RUST_CRC16_M17: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_M17);

#[allow(unused)]
const RUST_CRC16_MAXIM_DOW: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MAXIM_DOW);

#[allow(unused)]
const RUST_CRC16_MCRF4XX: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MCRF4XX);

#[allow(unused)]
const RUST_CRC16_MODBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_MODBUS);

#[allow(unused)]
const RUST_CRC16_NRSC_5: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_NRSC_5);

#[allow(unused)]
const RUST_CRC16_OPENSAFETY_A: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_OPENSAFETY_A);

#[allow(unused)]
const RUST_CRC16_OPENSAFETY_B: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_OPENSAFETY_B);

#[allow(unused)]
const RUST_CRC16_PROFIBUS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_PROFIBUS);

#[allow(unused)]
const RUST_CRC16_RIELLO: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_RIELLO);

#[allow(unused)]
const RUST_CRC16_SPI_FUJITSU: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_SPI_FUJITSU);

#[allow(unused)]
const RUST_CRC16_T10_DIF: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_T10_DIF);

#[allow(unused)]
const RUST_CRC16_TELEDISK: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_TELEDISK);

#[allow(unused)]
const RUST_CRC16_TMS37157: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_TMS37157);

#[allow(unused)]
const RUST_CRC16_UMTS: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_UMTS);

#[allow(unused)]
const RUST_CRC16_USB: crc::Crc<u16, Table<16>> = crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_USB);

#[allow(unused)]
const RUST_CRC16_XMODEM: crc::Crc<u16, Table<16>> =
    crc::Crc::<u16, Table<16>>::new(&crc::CRC_16_XMODEM);

#[allow(unused)]
const RUST_CRC32_AIXM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AIXM);

#[allow(unused)]
const RUST_CRC32_AUTOSAR: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AUTOSAR);

#[allow(unused)]
const RUST_CRC32_BASE91_D: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BASE91_D);

#[allow(unused)]
const RUST_CRC32_BZIP2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BZIP2);

#[allow(unused)]
const RUST_CRC32_CD_ROM_EDC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CD_ROM_EDC);

#[allow(unused)]
const RUST_CRC32_CKSUM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CKSUM);

#[allow(unused)]
const RUST_CRC32_ISCSI: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISCSI);

#[allow(unused)]
const RUST_CRC32_ISO_HDLC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISO_HDLC);

#[allow(unused)]
const RUST_CRC32_JAMCRC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_JAMCRC);

#[allow(unused)]
const RUST_CRC32_MEF: crc::Crc<u32, Table<16>> = crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MEF);

#[allow(unused)]
const RUST_CRC32_MPEG_2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MPEG_2);

#[allow(unused)]
const RUST_CRC32_XFER: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_XFER);

#[allow(unused)]
const RUST_CRC64_ECMA_182: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_ECMA_182);

#[allow(unused)]
const RUST_CRC64_GO_ISO: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_GO_ISO);

#[allow(unused)]
const RUST_CRC64_MS: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_MS);

#[allow(unused)]
const RUST_CRC64_NVME: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&CRC_64_NVME);

#[allow(unused)]
const RUST_CRC64_REDIS: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_REDIS);

#[allow(unused)]
const RUST_CRC64_WE: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_WE);

#[allow(unused)]
const RUST_CRC64_XZ: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_XZ);

#[allow(unused)]
// Dispatch function that handles the generic case
#[allow(deprecated)]
pub(crate) fn update(state: u64, data: &[u8], params: &CrcParams) -> u64 {
    match params.width {
        16 => {
            let params = match params.algorithm {
                CrcAlgorithm::Crc16Arc => RUST_CRC16_ARC,
                CrcAlgorithm::Crc16Cdma2000 => RUST_CRC16_CDMA2000,
                CrcAlgorithm::Crc16Cms => RUST_CRC16_CMS,
                CrcAlgorithm::Crc16Dds110 => RUST_CRC16_DDS_110,
                CrcAlgorithm::Crc16DectR => RUST_CRC16_DECT_R,
                CrcAlgorithm::Crc16DectX => RUST_CRC16_DECT_X,
                CrcAlgorithm::Crc16Dnp => RUST_CRC16_DNP,
                CrcAlgorithm::Crc16En13757 => RUST_CRC16_EN_13757,
                CrcAlgorithm::Crc16Genibus => RUST_CRC16_GENIBUS,
                CrcAlgorithm::Crc16Gsm => RUST_CRC16_GSM,
                CrcAlgorithm::Crc16Ibm3740 => RUST_CRC16_IBM_3740,
                CrcAlgorithm::Crc16IbmSdlc => RUST_CRC16_IBM_SDLC,
                CrcAlgorithm::Crc16IsoIec144433A => RUST_CRC16_ISO_IEC_14443_3_A,
                CrcAlgorithm::Crc16Kermit => RUST_CRC16_KERMIT,
                CrcAlgorithm::Crc16Lj1200 => RUST_CRC16_LJ1200,
                CrcAlgorithm::Crc16M17 => RUST_CRC16_M17,
                CrcAlgorithm::Crc16MaximDow => RUST_CRC16_MAXIM_DOW,
                CrcAlgorithm::Crc16Mcrf4xx => RUST_CRC16_MCRF4XX,
                CrcAlgorithm::Crc16Modbus => RUST_CRC16_MODBUS,
                CrcAlgorithm::Crc16Nrsc5 => RUST_CRC16_NRSC_5,
                CrcAlgorithm::Crc16OpensafetyA => RUST_CRC16_OPENSAFETY_A,
                CrcAlgorithm::Crc16OpensafetyB => RUST_CRC16_OPENSAFETY_B,
                CrcAlgorithm::Crc16Profibus => RUST_CRC16_PROFIBUS,
                CrcAlgorithm::Crc16Riello => RUST_CRC16_RIELLO,
                CrcAlgorithm::Crc16SpiFujitsu => RUST_CRC16_SPI_FUJITSU,
                CrcAlgorithm::Crc16T10Dif => RUST_CRC16_T10_DIF,
                CrcAlgorithm::Crc16Teledisk => RUST_CRC16_TELEDISK,
                CrcAlgorithm::Crc16Tms37157 => RUST_CRC16_TMS37157,
                CrcAlgorithm::Crc16Umts => RUST_CRC16_UMTS,
                CrcAlgorithm::Crc16Usb => RUST_CRC16_USB,
                CrcAlgorithm::Crc16Xmodem => RUST_CRC16_XMODEM,
                CrcAlgorithm::CrcCustom => {
                    #[cfg(feature = "alloc")]
                    {
                        extern crate alloc;
                        use alloc::boxed::Box;

                        #[cfg(any(feature = "std", feature = "cache"))]
                        {
                            let key: Crc16Key = (
                                params.poly as u16,
                                params.init as u16,
                                params.refin,
                                params.refout,
                                params.xorout as u16,
                                params.check as u16,
                            );

                            #[cfg(feature = "std")]
                            {
                                let cache =
                                    CUSTOM_CRC16_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock().unwrap();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u16> = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u16,
                                            init: params.init as u16,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u16,
                                            check: params.check as u16,
                                            residue: 0x0000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u16, Table<16>>::new(static_algorithm)
                            }

                            #[cfg(all(feature = "cache", not(feature = "std")))]
                            {
                                let cache =
                                    CUSTOM_CRC16_CACHE.call_once(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u16> = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u16,
                                            init: params.init as u16,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u16,
                                            check: params.check as u16,
                                            residue: 0x0000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u16, Table<16>>::new(static_algorithm)
                            }
                        }

                        // Without cache, just leak (no_std without cache feature)
                        #[cfg(not(any(feature = "std", feature = "cache")))]
                        {
                            let algorithm: Algorithm<u16> = Algorithm {
                                width: params.width,
                                poly: params.poly as u16,
                                init: params.init as u16,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout as u16,
                                check: params.check as u16,
                                residue: 0x0000,
                            };

                            let static_algorithm = Box::leak(Box::new(algorithm));

                            crc::Crc::<u16, Table<16>>::new(static_algorithm)
                        }
                    }
                    #[cfg(not(feature = "alloc"))]
                    panic!("Custom CRC parameters require the 'alloc' feature")
                }
                _ => panic!("Invalid algorithm for u16 CRC"),
            };
            update_u16(state as u16, data, params) as u64
        }
        32 => {
            let params = match params.algorithm {
                CrcAlgorithm::Crc32Aixm => RUST_CRC32_AIXM,
                CrcAlgorithm::Crc32Autosar => RUST_CRC32_AUTOSAR,
                CrcAlgorithm::Crc32Base91D => RUST_CRC32_BASE91_D,
                CrcAlgorithm::Crc32Bzip2 => RUST_CRC32_BZIP2,
                CrcAlgorithm::Crc32CdRomEdc => RUST_CRC32_CD_ROM_EDC,
                CrcAlgorithm::Crc32Cksum => RUST_CRC32_CKSUM,
                CrcAlgorithm::Crc32Iscsi => RUST_CRC32_ISCSI,
                CrcAlgorithm::Crc32IsoHdlc => RUST_CRC32_ISO_HDLC,
                CrcAlgorithm::Crc32Jamcrc => RUST_CRC32_JAMCRC,
                CrcAlgorithm::Crc32Mef => RUST_CRC32_MEF,
                CrcAlgorithm::Crc32Mpeg2 => RUST_CRC32_MPEG_2,
                CrcAlgorithm::Crc32Xfer => RUST_CRC32_XFER,
                CrcAlgorithm::Crc32Custom => {
                    #[cfg(feature = "alloc")]
                    {
                        extern crate alloc;
                        use alloc::boxed::Box;

                        // Use cache if std or cache feature is enabled
                        #[cfg(any(feature = "std", feature = "cache"))]
                        {
                            let key: Crc32Key = (
                                params.poly as u32,
                                params.init as u32,
                                params.refin,
                                params.refout,
                                params.xorout as u32,
                                params.check as u32,
                            );

                            #[cfg(feature = "std")]
                            {
                                let cache =
                                    CUSTOM_CRC32_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock().unwrap();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u32,
                                            init: params.init as u32,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u32,
                                            check: params.check as u32,
                                            residue: 0x00000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u32, Table<16>>::new(static_algorithm)
                            }

                            #[cfg(all(not(feature = "std"), feature = "cache"))]
                            {
                                let cache =
                                    CUSTOM_CRC32_CACHE.call_once(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u32,
                                            init: params.init as u32,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u32,
                                            check: params.check as u32,
                                            residue: 0x00000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u32, Table<16>>::new(static_algorithm)
                            }
                        }

                        // Without cache, just leak (no_std without cache feature)
                        #[cfg(not(any(feature = "std", feature = "cache")))]
                        {
                            let algorithm: Algorithm<u32> = Algorithm {
                                width: params.width,
                                poly: params.poly as u32,
                                init: params.init as u32,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout as u32,
                                check: params.check as u32,
                                residue: 0x00000000, // unused in this context
                            };

                            // ugly, but the crc crate is difficult to work with...
                            let static_algorithm = Box::leak(Box::new(algorithm));

                            crc::Crc::<u32, Table<16>>::new(static_algorithm)
                        }
                    }
                    #[cfg(not(feature = "alloc"))]
                    panic!("Custom CRC parameters require the 'alloc' feature")
                }
                CrcAlgorithm::CrcCustom => {
                    #[cfg(feature = "alloc")]
                    {
                        extern crate alloc;
                        use alloc::boxed::Box;

                        #[cfg(any(feature = "std", feature = "cache"))]
                        {
                            let key: Crc32Key = (
                                params.poly as u32,
                                params.init as u32,
                                params.refin,
                                params.refout,
                                params.xorout as u32,
                                params.check as u32,
                            );

                            #[cfg(feature = "std")]
                            {
                                let cache =
                                    CUSTOM_CRC32_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock().unwrap();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u32> = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u32,
                                            init: params.init as u32,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u32,
                                            check: params.check as u32,
                                            residue: 0x00000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u32, Table<16>>::new(static_algorithm)
                            }

                            #[cfg(all(feature = "cache", not(feature = "std")))]
                            {
                                let cache =
                                    CUSTOM_CRC32_CACHE.call_once(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u32> = Algorithm {
                                            width: params.width,
                                            poly: params.poly as u32,
                                            init: params.init as u32,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout as u32,
                                            check: params.check as u32,
                                            residue: 0x00000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u32, Table<16>>::new(static_algorithm)
                            }
                        }

                        // Without cache, just leak (no_std without cache feature)
                        #[cfg(not(any(feature = "std", feature = "cache")))]
                        {
                            let algorithm: Algorithm<u32> = Algorithm {
                                width: params.width,
                                poly: params.poly as u32,
                                init: params.init as u32,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout as u32,
                                check: params.check as u32,
                                residue: 0x00000000,
                            };

                            let static_algorithm = Box::leak(Box::new(algorithm));

                            crc::Crc::<u32, Table<16>>::new(static_algorithm)
                        }
                    }
                    #[cfg(not(feature = "alloc"))]
                    panic!("Custom CRC parameters require the 'alloc' feature")
                }
                _ => panic!("Invalid algorithm for u32 CRC"),
            };
            update_u32(state as u32, data, params) as u64
        }
        64 => {
            let params = match params.algorithm {
                CrcAlgorithm::Crc64Ecma182 => RUST_CRC64_ECMA_182,
                CrcAlgorithm::Crc64GoIso => RUST_CRC64_GO_ISO,
                CrcAlgorithm::Crc64Ms => RUST_CRC64_MS,
                CrcAlgorithm::Crc64Nvme => RUST_CRC64_NVME,
                CrcAlgorithm::Crc64Redis => RUST_CRC64_REDIS,
                CrcAlgorithm::Crc64We => RUST_CRC64_WE,
                CrcAlgorithm::Crc64Xz => RUST_CRC64_XZ,
                CrcAlgorithm::Crc64Custom => {
                    #[cfg(feature = "alloc")]
                    {
                        extern crate alloc;
                        use alloc::boxed::Box;

                        // Use cache if std or cache feature is enabled
                        #[cfg(any(feature = "std", feature = "cache"))]
                        {
                            let key: Crc64Key = (
                                params.poly,
                                params.init,
                                params.refin,
                                params.refout,
                                params.xorout,
                                params.check,
                            );

                            #[cfg(feature = "std")]
                            {
                                let cache =
                                    CUSTOM_CRC64_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock().unwrap();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm = Algorithm {
                                            width: params.width,
                                            poly: params.poly,
                                            init: params.init,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout,
                                            check: params.check,
                                            residue: 0x0000000000000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u64, Table<16>>::new(static_algorithm)
                            }

                            #[cfg(all(not(feature = "std"), feature = "cache"))]
                            {
                                let cache =
                                    CUSTOM_CRC64_CACHE.call_once(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm = Algorithm {
                                            width: params.width,
                                            poly: params.poly,
                                            init: params.init,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout,
                                            check: params.check,
                                            residue: 0x0000000000000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u64, Table<16>>::new(static_algorithm)
                            }
                        }

                        // Without cache, just leak (no_std without cache feature)
                        #[cfg(not(any(feature = "std", feature = "cache")))]
                        {
                            let algorithm: Algorithm<u64> = Algorithm {
                                width: params.width,
                                poly: params.poly,
                                init: params.init,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout,
                                check: params.check,
                                residue: 0x0000000000000000, // unused in this context
                            };

                            // ugly, but the crc crate is difficult to work with...
                            let static_algorithm = Box::leak(Box::new(algorithm));

                            crc::Crc::<u64, Table<16>>::new(static_algorithm)
                        }
                    }
                    #[cfg(not(feature = "alloc"))]
                    panic!("Custom CRC parameters require the 'alloc' feature")
                }
                CrcAlgorithm::CrcCustom => {
                    #[cfg(feature = "alloc")]
                    {
                        extern crate alloc;
                        use alloc::boxed::Box;

                        #[cfg(any(feature = "std", feature = "cache"))]
                        {
                            let key: Crc64Key = (
                                params.poly,
                                params.init,
                                params.refin,
                                params.refout,
                                params.xorout,
                                params.check,
                            );

                            #[cfg(feature = "std")]
                            {
                                let cache =
                                    CUSTOM_CRC64_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock().unwrap();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u64> = Algorithm {
                                            width: params.width,
                                            poly: params.poly,
                                            init: params.init,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout,
                                            check: params.check,
                                            residue: 0x0000000000000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u64, Table<16>>::new(static_algorithm)
                            }

                            #[cfg(all(feature = "cache", not(feature = "std")))]
                            {
                                let cache =
                                    CUSTOM_CRC64_CACHE.call_once(|| Mutex::new(HashMap::new()));
                                let mut cache_guard = cache.lock();

                                let static_algorithm =
                                    cache_guard.entry(key).or_insert_with(|| {
                                        let algorithm: Algorithm<u64> = Algorithm {
                                            width: params.width,
                                            poly: params.poly,
                                            init: params.init,
                                            refin: params.refin,
                                            refout: params.refout,
                                            xorout: params.xorout,
                                            check: params.check,
                                            residue: 0x0000000000000000,
                                        };
                                        Box::leak(Box::new(algorithm))
                                    });

                                crc::Crc::<u64, Table<16>>::new(static_algorithm)
                            }
                        }

                        // Without cache, just leak (no_std without cache feature)
                        #[cfg(not(any(feature = "std", feature = "cache")))]
                        {
                            let algorithm: Algorithm<u64> = Algorithm {
                                width: params.width,
                                poly: params.poly,
                                init: params.init,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout,
                                check: params.check,
                                residue: 0x0000000000000000,
                            };

                            let static_algorithm = Box::leak(Box::new(algorithm));

                            crc::Crc::<u64, Table<16>>::new(static_algorithm)
                        }
                    }
                    #[cfg(not(feature = "alloc"))]
                    panic!("Custom CRC parameters require the 'alloc' feature")
                }
                _ => panic!("Invalid algorithm for u64 CRC"),
            };
            update_u64(state, data, params)
        }
        _ => panic!("Unsupported CRC width: {}", params.width),
    }
}

// Specific implementation for u16
fn update_u16(state: u16, data: &[u8], params: crc::Crc<u16, Table<16>>) -> u16 {
    // apply REFIN if necessary
    let initial = if params.algorithm.refin {
        state.reverse_bits()
    } else {
        state
    };

    let mut digest = params.digest_with_initial(initial);
    digest.update(data);

    let checksum = digest.finalize();

    // remove XOR since this will be applied in the library Digest::finalize() step instead
    checksum ^ params.algorithm.xorout
}

// Specific implementation for u32
fn update_u32(state: u32, data: &[u8], params: crc::Crc<u32, Table<16>>) -> u32 {
    // apply REFIN if necessary
    let initial = if params.algorithm.refin {
        state.reverse_bits()
    } else {
        state
    };

    let mut digest = params.digest_with_initial(initial);
    digest.update(data);

    let checksum = digest.finalize();

    // remove XOR since this will be applied in the library Digest::finalize() step instead
    checksum ^ params.algorithm.xorout
}

// Specific implementation for u64
fn update_u64(state: u64, data: &[u8], params: crc::Crc<u64, Table<16>>) -> u64 {
    // apply REFIN if necessary
    let initial = if params.algorithm.refin {
        state.reverse_bits()
    } else {
        state
    };

    let mut digest = params.digest_with_initial(initial);
    digest.update(data);

    // remove XOR since this will be applied in the library Digest::finalize() step instead
    digest.finalize() ^ params.algorithm.xorout
}
