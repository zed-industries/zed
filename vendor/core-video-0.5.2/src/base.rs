use libc::c_double;

pub type CVOptionFlags = u64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CVSMPTETime {
    pub subframes: i16,
    pub subframeDivisor: i16,
    pub counter: u32,
    pub type_: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CVSMPTETimeType {
    kCVSMPTETimeType24       = 0,
    kCVSMPTETimeType25       = 1,
    kCVSMPTETimeType30Drop   = 2,
    kCVSMPTETimeType30       = 3,
    kCVSMPTETimeType2997     = 4,
    kCVSMPTETimeType2997Drop = 5,
    kCVSMPTETimeType60       = 6,
    kCVSMPTETimeType5994     = 7,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CVSMPTETimeFlags {
    kCVSMPTETimeValid   = (1 << 0),
    kCVSMPTETimeRunning = (1 << 1),
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CVTimeFlags {
    kCVTimeIsIndefinite = (1 << 0),
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CVTime {
    pub timeValue: i64,
    pub timeScale: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: c_double,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CVTimeStampFlags {
    kCVTimeStampVideoTimeValid     = (1 << 0),
    kCVTimeStampHostTimeValid      = (1 << 1),
    kCVTimeStampSMPTETimeValid     = (1 << 2),
    kCVTimeStampVideoRefreshPeriodValid = (1 << 3),
    kCVTimeStampRateScalarValid    = (1 << 4),
    kCVTimeStampTopField           = (1 << 16),
    kCVTimeStampBottomField        = (1 << 17),
    kCVTimeStampVideoHostTimeValid = (1 << 0) | (1 << 1), /* kCVTimeStampVideoTimeValid |
                                                           * kCVTimeStampHostTimeValid */
    kCVTimeStampIsInterlaced       = (1 << 16) | (1 << 17), /* kCVTimeStampTopField |
                                                             * kCVTimeStampBottomField */
}

extern "C" {
    pub static kCVZeroTime: CVTime;
    pub static kCVIndefiniteTime: CVTime;
}
