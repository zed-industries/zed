use libc::c_double;

// https://developer.apple.com/documentation/corevideo/cvoptionflags?language=objc
pub type CVOptionFlags = u64;
pub type CVSMPTETimeType = u32;
pub type CVSMPTETimeFlags = u32;
pub type CVTimeFlags = i32;
pub type CVTimeStampFlags = u64;

pub const kCVSMPTETimeType24: CVSMPTETimeType = 0;
pub const kCVSMPTETimeType25: CVSMPTETimeType = 1;
pub const kCVSMPTETimeType30Drop: CVSMPTETimeType = 2;
pub const kCVSMPTETimeType30: CVSMPTETimeType = 3;
pub const kCVSMPTETimeType2997: CVSMPTETimeType = 4;
pub const kCVSMPTETimeType2997Drop: CVSMPTETimeType = 5;
pub const kCVSMPTETimeType60: CVSMPTETimeType = 6;
pub const kCVSMPTETimeType5994: CVSMPTETimeType = 7;

pub const kCVSMPTETimeValid: CVSMPTETimeFlags = 1 << 0;
pub const kCVSMPTETimeRunning: CVSMPTETimeFlags = 1 << 1;

pub const kCVTimeIsIndefinite: CVTimeFlags = 1 << 0;

#[repr(C)]
#[derive(Debug, Clone)]
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CVTime {
    pub timeValue: i64,
    pub timeScale: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
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

pub const kCVTimeStampVideoTimeValid: CVTimeStampFlags = 1 << 0;
pub const kCVTimeStampHostTimeValid: CVTimeStampFlags = 1 << 1;
pub const kCVTimeStampSMPTETimeValid: CVTimeStampFlags = 1 << 2;
pub const kCVTimeStampVideoRefreshPeriodValid: CVTimeStampFlags = 1 << 3;
pub const kCVTimeStampRateScalarValid: CVTimeStampFlags = 1 << 4;

// There are flags for each field to make it easier to detect interlaced vs progressive output
pub const kCVTimeStampTopField: CVTimeStampFlags = 1 << 16;
pub const kCVTimeStampBottomField: CVTimeStampFlags = 1 << 17;

// Some commonly used combinations of timestamp flags
pub const kCVTimeStampVideoHostTimeValid: CVTimeStampFlags =
    kCVTimeStampVideoTimeValid | kCVTimeStampHostTimeValid;
pub const kCVTimeStampIsInterlaced: CVTimeStampFlags =
    kCVTimeStampTopField | kCVTimeStampBottomField;

extern "C" {
    pub static kCVZeroTime: CVTime;
    pub static kCVIndefiniteTime: CVTime;
}
