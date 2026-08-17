// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

use crate::consts::*;
use crate::CrcAlgorithm;
use core::fmt::{Display, Formatter};
use core::str::FromStr;

impl FromStr for CrcAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NAME_CRC16_ARC => Ok(CrcAlgorithm::Crc16Arc),
            NAME_CRC16_CDMA2000 => Ok(CrcAlgorithm::Crc16Cdma2000),
            NAME_CRC16_CMS => Ok(CrcAlgorithm::Crc16Cms),
            NAME_CRC16_DDS_110 => Ok(CrcAlgorithm::Crc16Dds110),
            NAME_CRC16_DECT_R => Ok(CrcAlgorithm::Crc16DectR),
            NAME_CRC16_DECT_X => Ok(CrcAlgorithm::Crc16DectX),
            NAME_CRC16_DNP => Ok(CrcAlgorithm::Crc16Dnp),
            NAME_CRC16_EN_13757 => Ok(CrcAlgorithm::Crc16En13757),
            NAME_CRC16_GENIBUS => Ok(CrcAlgorithm::Crc16Genibus),
            NAME_CRC16_GSM => Ok(CrcAlgorithm::Crc16Gsm),
            NAME_CRC16_IBM_3740 => Ok(CrcAlgorithm::Crc16Ibm3740),
            NAME_CRC16_IBM_SDLC => Ok(CrcAlgorithm::Crc16IbmSdlc),
            NAME_CRC16_ISO_IEC_14443_3_A => Ok(CrcAlgorithm::Crc16IsoIec144433A),
            NAME_CRC16_KERMIT => Ok(CrcAlgorithm::Crc16Kermit),
            NAME_CRC16_LJ1200 => Ok(CrcAlgorithm::Crc16Lj1200),
            NAME_CRC16_M17 => Ok(CrcAlgorithm::Crc16M17),
            NAME_CRC16_MAXIM_DOW => Ok(CrcAlgorithm::Crc16MaximDow),
            NAME_CRC16_MCRF4XX => Ok(CrcAlgorithm::Crc16Mcrf4xx),
            NAME_CRC16_MODBUS => Ok(CrcAlgorithm::Crc16Modbus),
            NAME_CRC16_NRSC_5 => Ok(CrcAlgorithm::Crc16Nrsc5),
            NAME_CRC16_OPENSAFETY_A => Ok(CrcAlgorithm::Crc16OpensafetyA),
            NAME_CRC16_OPENSAFETY_B => Ok(CrcAlgorithm::Crc16OpensafetyB),
            NAME_CRC16_PROFIBUS => Ok(CrcAlgorithm::Crc16Profibus),
            NAME_CRC16_RIELLO => Ok(CrcAlgorithm::Crc16Riello),
            NAME_CRC16_SPI_FUJITSU => Ok(CrcAlgorithm::Crc16SpiFujitsu),
            NAME_CRC16_T10_DIF => Ok(CrcAlgorithm::Crc16T10Dif),
            NAME_CRC16_TELEDISK => Ok(CrcAlgorithm::Crc16Teledisk),
            NAME_CRC16_TMS37157 => Ok(CrcAlgorithm::Crc16Tms37157),
            NAME_CRC16_UMTS => Ok(CrcAlgorithm::Crc16Umts),
            NAME_CRC16_USB => Ok(CrcAlgorithm::Crc16Usb),
            NAME_CRC16_XMODEM => Ok(CrcAlgorithm::Crc16Xmodem),
            NAME_CRC32_AIXM => Ok(CrcAlgorithm::Crc32Aixm),
            NAME_CRC32_AUTOSAR => Ok(CrcAlgorithm::Crc32Autosar),
            NAME_CRC32_BASE91_D => Ok(CrcAlgorithm::Crc32Base91D),
            NAME_CRC32_BZIP2 => Ok(CrcAlgorithm::Crc32Bzip2),
            NAME_CRC32_CD_ROM_EDC => Ok(CrcAlgorithm::Crc32CdRomEdc),
            NAME_CRC32_CKSUM => Ok(CrcAlgorithm::Crc32Cksum),
            NAME_CRC32_ISCSI => Ok(CrcAlgorithm::Crc32Iscsi),
            NAME_CRC32_ISO_HDLC => Ok(CrcAlgorithm::Crc32IsoHdlc),
            NAME_CRC32_JAMCRC => Ok(CrcAlgorithm::Crc32Jamcrc),
            NAME_CRC32_MEF => Ok(CrcAlgorithm::Crc32Mef),
            NAME_CRC32_MPEG_2 => Ok(CrcAlgorithm::Crc32Mpeg2),
            NAME_CRC32_XFER => Ok(CrcAlgorithm::Crc32Xfer),
            NAME_CRC64_GO_ISO => Ok(CrcAlgorithm::Crc64GoIso),
            NAME_CRC64_MS => Ok(CrcAlgorithm::Crc64Ms),
            NAME_CRC64_NVME => Ok(CrcAlgorithm::Crc64Nvme),
            NAME_CRC64_REDIS => Ok(CrcAlgorithm::Crc64Redis),
            NAME_CRC64_XZ => Ok(CrcAlgorithm::Crc64Xz),
            NAME_CRC64_ECMA_182 => Ok(CrcAlgorithm::Crc64Ecma182),
            NAME_CRC64_WE => Ok(CrcAlgorithm::Crc64We),
            _ => Err(()),
        }
    }
}

#[allow(deprecated)]
impl Display for CrcAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            CrcAlgorithm::Crc16Arc => write!(f, "{NAME_CRC16_ARC}"),
            CrcAlgorithm::Crc16Cdma2000 => write!(f, "{NAME_CRC16_CDMA2000}"),
            CrcAlgorithm::Crc16Cms => write!(f, "{NAME_CRC16_CMS}"),
            CrcAlgorithm::Crc16Dds110 => write!(f, "{NAME_CRC16_DDS_110}"),
            CrcAlgorithm::Crc16DectR => write!(f, "{NAME_CRC16_DECT_R}"),
            CrcAlgorithm::Crc16DectX => write!(f, "{NAME_CRC16_DECT_X}"),
            CrcAlgorithm::Crc16Dnp => write!(f, "{NAME_CRC16_DNP}"),
            CrcAlgorithm::Crc16En13757 => write!(f, "{NAME_CRC16_EN_13757}"),
            CrcAlgorithm::Crc16Genibus => write!(f, "{NAME_CRC16_GENIBUS}"),
            CrcAlgorithm::Crc16Gsm => write!(f, "{NAME_CRC16_GSM}"),
            CrcAlgorithm::Crc16Ibm3740 => write!(f, "{NAME_CRC16_IBM_3740}",),
            CrcAlgorithm::Crc16IbmSdlc => write!(f, "{NAME_CRC16_IBM_SDLC}",),
            CrcAlgorithm::Crc16IsoIec144433A => {
                write!(f, "{NAME_CRC16_ISO_IEC_14443_3_A}",)
            }
            CrcAlgorithm::Crc16Kermit => write!(f, "{NAME_CRC16_KERMIT}"),
            CrcAlgorithm::Crc16Lj1200 => write!(f, "{NAME_CRC16_LJ1200}"),
            CrcAlgorithm::Crc16M17 => write!(f, "{NAME_CRC16_M17}"),
            CrcAlgorithm::Crc16MaximDow => write!(f, "{NAME_CRC16_MAXIM_DOW}"),
            CrcAlgorithm::Crc16Mcrf4xx => write!(f, "{NAME_CRC16_MCRF4XX}"),
            CrcAlgorithm::Crc16Modbus => write!(f, "{NAME_CRC16_MODBUS}"),
            CrcAlgorithm::Crc16Nrsc5 => write!(f, "{NAME_CRC16_NRSC_5}"),
            CrcAlgorithm::Crc16OpensafetyA => write!(f, "{NAME_CRC16_OPENSAFETY_A}"),
            CrcAlgorithm::Crc16OpensafetyB => write!(f, "{NAME_CRC16_OPENSAFETY_B}"),
            CrcAlgorithm::Crc16Profibus => write!(f, "{NAME_CRC16_PROFIBUS}"),
            CrcAlgorithm::Crc16Riello => write!(f, "{NAME_CRC16_RIELLO}"),
            CrcAlgorithm::Crc16SpiFujitsu => write!(f, "{NAME_CRC16_SPI_FUJITSU}"),
            CrcAlgorithm::Crc16T10Dif => write!(f, "{NAME_CRC16_T10_DIF}",),
            CrcAlgorithm::Crc16Teledisk => write!(f, "{NAME_CRC16_TELEDISK}"),
            CrcAlgorithm::Crc16Tms37157 => write!(f, "{NAME_CRC16_TMS37157}"),
            CrcAlgorithm::Crc16Umts => write!(f, "{NAME_CRC16_UMTS}"),
            CrcAlgorithm::Crc16Usb => write!(f, "{NAME_CRC16_USB}"),
            CrcAlgorithm::Crc16Xmodem => write!(f, "{NAME_CRC16_XMODEM}"),
            CrcAlgorithm::Crc32Aixm => write!(f, "{NAME_CRC32_AIXM}",),
            CrcAlgorithm::Crc32Autosar => write!(f, "{NAME_CRC32_AUTOSAR}",),
            CrcAlgorithm::Crc32Base91D => write!(f, "{NAME_CRC32_BASE91_D}",),
            CrcAlgorithm::Crc32Bzip2 => write!(f, "{NAME_CRC32_BZIP2}",),
            CrcAlgorithm::Crc32CdRomEdc => write!(f, "{NAME_CRC32_CD_ROM_EDC}",),
            CrcAlgorithm::Crc32Cksum => write!(f, "{NAME_CRC32_CKSUM}",),
            CrcAlgorithm::Crc32Custom => write!(f, "CRC-32/CUSTOM"),
            CrcAlgorithm::Crc32Iscsi => write!(f, "{NAME_CRC32_ISCSI}",),
            CrcAlgorithm::Crc32IsoHdlc => write!(f, "{NAME_CRC32_ISO_HDLC}",),
            CrcAlgorithm::Crc32Jamcrc => write!(f, "{NAME_CRC32_JAMCRC}",),
            CrcAlgorithm::Crc32Mef => write!(f, "{NAME_CRC32_MEF}",),
            CrcAlgorithm::Crc32Mpeg2 => write!(f, "{NAME_CRC32_MPEG_2}",),
            CrcAlgorithm::Crc32Xfer => write!(f, "{NAME_CRC32_XFER}",),
            CrcAlgorithm::CrcCustom => write!(f, "CRC/CUSTOM"),
            CrcAlgorithm::Crc64Custom => write!(f, "CRC-64/CUSTOM"),
            CrcAlgorithm::Crc64GoIso => write!(f, "{NAME_CRC64_GO_ISO}",),
            CrcAlgorithm::Crc64Ms => write!(f, "{NAME_CRC64_MS}",),
            CrcAlgorithm::Crc64Nvme => write!(f, "{NAME_CRC64_NVME}",),
            CrcAlgorithm::Crc64Redis => write!(f, "{NAME_CRC64_REDIS}",),
            CrcAlgorithm::Crc64Xz => write!(f, "{NAME_CRC64_XZ}",),
            CrcAlgorithm::Crc64Ecma182 => write!(f, "{NAME_CRC64_ECMA_182}",),
            CrcAlgorithm::Crc64We => write!(f, "{NAME_CRC64_WE}",),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Reflector<T> {
    NoReflector,
    ForwardReflector { smask: T },
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
/// Different processing strategies based on data length
pub(crate) enum DataChunkProcessor {
    From0To15,   // 0-15 bytes
    From16,      // exactly 16 bytes
    From17To31,  // 17-31 bytes
    From32To255, // 32-255 bytes
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
impl DataChunkProcessor {
    /// Select the appropriate processor based on data length
    pub fn for_length(len: usize) -> Self {
        match len {
            0..=15 => Self::From0To15,
            16 => Self::From16,
            17..=31 => Self::From17To31,
            32..=255 => Self::From32To255,
            _ => panic!("data length too large"),
        }
    }
}
